# Tailwind CSS

Gram has built-in support for Tailwind CSS autocomplete, linting, and hover previews.

- Language Server: [tailwindlabs/tailwindcss-intellisense](https://github.com/tailwindlabs/tailwindcss-intellisense)

## Configuration

To configure the Tailwind CSS language server, refer [to the extension settings](https://github.com/tailwindlabs/tailwindcss-intellisense?tab=readme-ov-file#extension-settings) and add them to the `lsp` section of your `settings.json`:

```jsonc
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "classFunctions": ["cva", "cx"],
        "experimental": {
          "classRegex": ["[cls|className]\\s\\:\\=\\s\"([^\"]*)"],
        },
      },
    },
  },
}
```

Languages which can be used with Tailwind CSS in Gram:

- [Astro](gram://docs/languages/astro)
- [CSS](gram://docs/languages/css)
- [ERB](gram://docs/languages/ruby)
- [Gleam](gram://docs/languages/gleam)
- [HEEx](gram://docs/languages/elixir#heex)
- [HTML](gram://docs/languages/html)
- [TypeScript](gram://docs/languages/typescript)
- [JavaScript](gram://docs/languages/javascript)
- [PHP](gram://docs/languages/php)
- [Svelte](gram://docs/languages/svelte)
- [Vue](gram://docs/languages/vue)

### Prettier Plugin

Gram supports Prettier out of the box, which means that if you have the [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) installed, adding it to your Prettier configuration will make it work automatically:

```jsonc
// .prettierrc
{
  "plugins": ["prettier-plugin-tailwindcss"],
}
```
