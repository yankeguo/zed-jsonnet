# zed-jsonnet

[Jsonnet][1] language support for [Zed][2] editor.

![demo-syntax-jsonnet](./static/demo-syntax-macos-light.png)

## Settings

The [jsonnet-language-server][] settings can be changed in the `lsp` section of your settings.json.

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

The extension downloads the [jsonnet-language-server][] binary from GitHub releases.
Unauthenticated requests to the GitHub API are [rate limited][4], which may cause the
download to fail with an "API rate limit exceeded" error.

To avoid this, you can export a [GitHub personal access token][5] (no scopes are
required for public repositories) as the `GITHUB_TOKEN` environment variable in your
shell profile (e.g. `~/.zshrc`):

```
export GITHUB_TOKEN=ghp_...
```

The extension reads the token from your shell environment, so it never has to be
stored in plain text in `settings.json`. The token is only used by the extension to
query the latest release and is never passed to the language server.

## Related Projects

- [tree-sitter-jsonnet](https://github.com/sourcegraph/tree-sitter-jsonnet)
- [jsonnet-language-server](https://github.com/grafana/jsonnet-language-server)

## Development

Refer to Zed's "[Developing Extensions][3]" documentation.

### Troubleshooting

1. Clean up the workspace with `make distclean`.
2. ...

### New version

```
% ./scripts/bump-version.sh <version>
% git commit -m "Release <version>" --all
```

[1]: https://jsonnet.org/
[2]: https://zed.dev/
[3]: https://zed.dev/docs/extensions/developing-extensions
[4]: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api
[5]: https://github.com/settings/tokens
[jsonnet-language-server]: https://github.com/grafana/jsonnet-language-server
