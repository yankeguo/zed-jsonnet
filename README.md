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

To avoid this, you can configure an optional [GitHub personal access token][5] (no
scopes are required for public repositories) in the `initialization_options` section.
The token is only used by the extension to query the latest release and is never
passed to the language server:

```
{
  "lsp": {
    "jsonnet-language-server": {
      "initialization_options": {
        "github_token": "ghp_..."
      }
    }
  }
}
```

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
