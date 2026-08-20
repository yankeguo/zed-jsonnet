use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod, HttpRequest},
    serde_json,
    settings::LspSettings,
    Result,
};

const LSP_GITHUB_REPO: &str = "grafana/jsonnet-language-server";
const GITHUB_TOKEN_ENV_VAR: &str = "GITHUB_TOKEN";
const ZED_JSONNET_GITHUB_TOKEN_ENV_VAR: &str = "ZED_JSONNET_GITHUB_TOKEN";

/// Returns a GitHub token from the `gh` CLI, if it is installed and
/// authenticated.
///
/// The extension runs inside a sandboxed WASI environment, so the user's
/// shell environment is forwarded explicitly to let the host resolve `gh`
/// on the user's `PATH`.
fn gh_cli_token(worktree: &zed::Worktree) -> Option<String> {
    let output = zed::process::Command::new("gh")
        .arg("auth")
        .arg("token")
        .envs(worktree.shell_env())
        .output()
        .map_err(|err| eprintln!("zed-jsonnet: failed to run `gh auth token`: {err}"))
        .ok()?;
    if output.status != Some(0) {
        eprintln!(
            "zed-jsonnet: `gh auth token` exited with status {:?}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    let token = String::from_utf8(output.stdout).ok()?;
    let token = token.trim();
    (!token.is_empty()).then(|| token.to_string())
}

/// Returns the named environment variable from the user's shell environment,
/// if set and non-empty.
///
/// The Zed extension API offers no secure credential storage, so the token is
/// read from the shell environment instead of being stored in plain text in
/// `settings.json`.
fn shell_env_var(worktree: &zed::Worktree, name: &str) -> Option<String> {
    worktree
        .shell_env()
        .into_iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value)
        .filter(|value| !value.is_empty())
}

/// Fetches the latest GitHub release through the GitHub REST API,
/// authenticated with the given token to avoid rate limiting.
fn latest_github_release_with_token(token: &str) -> Result<zed::GithubRelease> {
    let response = HttpRequest::builder()
        .method(HttpMethod::Get)
        .url(format!(
            "https://api.github.com/repos/{LSP_GITHUB_REPO}/releases/latest"
        ))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "zed-jsonnet")
        .build()
        .map_err(|err| format!("failed to build GitHub release request: {err}"))?
        .fetch()
        .map_err(|err| format!("failed to fetch latest GitHub release: {err}"))?;

    let body: serde_json::Value = serde_json::from_slice(&response.body)
        .map_err(|err| format!("failed to parse GitHub release response: {err}"))?;

    let version = body
        .get("tag_name")
        .and_then(|tag_name| tag_name.as_str())
        .ok_or_else(|| {
            let message = body
                .get("message")
                .and_then(|message| message.as_str())
                .unwrap_or("unexpected response");
            format!("failed to fetch latest GitHub release: {message}")
        })?
        .to_string();

    let assets = body
        .get("assets")
        .and_then(|assets| assets.as_array())
        .map(|assets| {
            assets
                .iter()
                .filter_map(|asset| {
                    Some(zed::GithubReleaseAsset {
                        name: asset.get("name")?.as_str()?.to_string(),
                        download_url: asset.get("browser_download_url")?.as_str()?.to_string(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if assets.is_empty() {
        return Err("latest GitHub release has no assets".to_string());
    }

    Ok(zed::GithubRelease { version, assets })
}

struct JsonnetExtension {
    cached_binary_path: Option<String>,
}

impl JsonnetExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let (token, token_source) = gh_cli_token(worktree)
            .map(|token| (token, "gh CLI (`gh auth token`)"))
            .or_else(|| {
                shell_env_var(worktree, ZED_JSONNET_GITHUB_TOKEN_ENV_VAR)
                    .map(|token| (token, "ZED_JSONNET_GITHUB_TOKEN environment variable"))
            })
            .or_else(|| {
                shell_env_var(worktree, GITHUB_TOKEN_ENV_VAR)
                    .map(|token| (token, "GITHUB_TOKEN environment variable"))
            })
            .unzip();
        let release = match token {
            Some(token) => {
                let token_source = token_source.unwrap_or("unknown source");
                eprintln!(
                    "zed-jsonnet: fetching latest release with GitHub token from {token_source}"
                );
                latest_github_release_with_token(&token).map_err(|err| {
                    format!("{err} (authenticated with GitHub token from {token_source})")
                })?
            }
            None => {
                eprintln!(
                    "zed-jsonnet: no GitHub token found (tried gh CLI, \
                     {ZED_JSONNET_GITHUB_TOKEN_ENV_VAR}, {GITHUB_TOKEN_ENV_VAR}); \
                     using unauthenticated GitHub API, subject to rate limiting"
                );
                zed::latest_github_release(
                    LSP_GITHUB_REPO,
                    zed::GithubReleaseOptions {
                        require_assets: true,
                        pre_release: false,
                    },
                )
                .map_err(|err| {
                    format!(
                        "{err} (no GitHub token found: tried gh CLI, \
                         {ZED_JSONNET_GITHUB_TOKEN_ENV_VAR}, {GITHUB_TOKEN_ENV_VAR}; \
                         unauthenticated GitHub API is rate-limited per IP)"
                    )
                })?
            }
        };

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "jsonnet-language-server_{version}_{os}_{arch}{extension}",
            version = release
                .version
                .strip_prefix("v")
                .unwrap_or(release.version.as_str()),
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "amd64",
                zed::Architecture::X86 => return Err(format!("no asset for arch '{arch:?}'")),
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            extension = match platform {
                zed::Os::Windows => ".exe",
                zed::Os::Mac | zed::Os::Linux => "",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("jsonnet-language-server-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        let binary_path = format!("{version_dir}/jsonnet-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for JsonnetExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["--log-level".to_string(), "info".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(JsonnetExtension);
