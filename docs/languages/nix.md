# Nix

Nix support is built into the editor.

## Choosing a language server

There is support for two different language servers: `nil` and `nixd`.

To only enable `nil`:

```jsonc
  "languages": {
    "Nix": {
      "language_servers": ["nil", "!nixd", "..."]
    },
  }
```

To only enable `nixd`:

```jsonc
  "languages": {
    "Nix": {
      "language_servers": ["!nil", "nixd", "..."]
    },
  }
```
