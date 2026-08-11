# zed-jsonnet

[Jsonnet][1] language support for the [Zed][2] editor.

This project is a hard fork of [narqo/zed-jsonnet][6], originally created by
Vladimir Varankin <vladimir@varank.in>, and continues to be distributed under
the Apache License 2.0.

## Installation

This extension is not published to the Zed extension registry and is intended
to be installed as a [dev extension][3]:

1. Clone this repository:

   ```
   % git clone https://github.com/yankeguo/zed-jsonnet
   ```

2. In Zed, open the command palette (`cmd-shift-p`), run
   `zed: install dev extension` (or click the **Install Dev Extension** button
   on the Extensions page) and select the cloned directory.

Zed will compile the extension and the tree-sitter grammar automatically. If
you have a published version of a Jsonnet extension installed, it will be
overridden by the dev extension. If something goes wrong, check the log with
`zed: open log`.

To update the extension later, pull the repository and run
`zed: install dev extension` again on the same directory.

## Features

- Syntax highlighting, code folding, outline, auto-indentation and bracket
  matching powered by [tree-sitter-jsonnet][7]
- Full language server integration (completion, go-to-definition, diagnostics,
  hover) via [jsonnet-language-server][], automatically downloaded and kept up
  to date
- Function, object and comment text objects in Vim mode

## Settings

The [jsonnet-language-server][] settings can be changed in the `lsp` section of
your settings.json.

```
{
  "lsp": {
    "jsonnet-language-server": {
      "settings": {
        "log_level": "info",
        "resolve_paths_with_tanka": true
      }
    }
  }
}
```

### GitHub Token

The extension downloads the [jsonnet-language-server][] binary from GitHub
releases. Unauthenticated requests to the GitHub API are [rate limited][4],
which may cause the download to fail with an "API rate limit exceeded" error.

To avoid this, you can export a [GitHub personal access token][5] (no scopes
are required for public repositories) as the `GITHUB_TOKEN` environment
variable in your shell profile (e.g. `~/.zshrc`):

```
export GITHUB_TOKEN=ghp_...
```

The extension reads the token from your shell environment, so it never has to
be stored in plain text in `settings.json`. The token is only used by the
extension to query the latest release and is never passed to the language
server.

## Development

Refer to Zed's "[Developing Extensions][3]" documentation.

To clean up build artifacts, remove the `target/`, `grammars/` and
`extension.wasm` paths. To cut a new version, bump the `version` field in
both `extension.toml` and `Cargo.toml`.

[1]: https://jsonnet.org/
[2]: https://zed.dev/
[3]: https://zed.dev/docs/extensions/developing-extensions
[4]: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api
[5]: https://github.com/settings/tokens
[6]: https://github.com/narqo/zed-jsonnet
[7]: https://github.com/sourcegraph/tree-sitter-jsonnet
[jsonnet-language-server]: https://github.com/grafana/jsonnet-language-server
