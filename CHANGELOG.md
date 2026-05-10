# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking

- New website: <https://gram-editor.com>
- Modified default settings to be more sensible (#184)
- Downloaded language servers no longer auto update (#267)
- Emmyluadoc tree-sitter grammar was removed (it was broken)
- Remove download support for superhtml on Linux (#194) (it was broken)

### Added

- Add "Tight" line spacing for project panel
- Add Arch dependency check (#189) by @theDoctor
- Add Arch package build (#177) by @theDoctor
- Add optional FS poll watcher (based on [poll])
- Add tree-sitter grammar for git rebase
- RPM repository now available
- Add Refresh Folders command (#207) (zed#46291)
- Add SelectInsideDelimiters and SelectAroundDelimiters actions (#217) by @subeax
- Use patched protobuf-src for Windows support (#202) by @voltagex
- Added a menu item in the buffer and tab right-click menu for
  opening Markdown and SVG files in the preview tab. (zed#47821)
- Add ability to rename terminal tabs (zed#45800)
- Add Mermaid diagram support to Markdown preview (zed#49064)
- Add ScrollToTop and ScrollToBottom actions in Markdown preview (zed#50460)
- Add Kintsugi theme via [Giacomo Cavalieri][kintsugi]
- Add support for natively installed ESLint and Typescript LSPs (#229) by @brib
- Autoclose quotes in SQL (#183)
- Add ctrl-x g to toggle Git panel in Emacs keymap (#154)
- Update Arch Linux in readme; Add Alpine Linux (#249) by @msrd0
- Open file by absolute path in file finder (#212)
- Add an option to hide the unsaved indicator (#266) by @bux
- Make LSP auto-updating an opt-in setting (#267) by @bux
- Add option to move status bar to top of window
- Add ctrl/cmd-shift-o key binding to open tab switcher
- Improve support for VSCodium task/debug import (#261)
- Smooth scrolling animation support (#140) (zed#44827)
- Show active file name in the status bar (zed#52381)

[poll]: https://github.com/lilith/zed/commit/0334469a57a20586b28b86187028acd36559a9d3
[kintsugi]: https://github.com/giacomocavalieri/kintsugi-zed

### Removed

- Remove unimplemented CLI dev_server_token argument
- Remove emmyluadoc tree-sitter grammar; it can be pulled in via extension instead
- Remove avatar from git graph
- Remove download support for superhtml on Linux (#194)
- Remove Windows OpenConsole.exe binary from source code tree
- Removed unused message_editor and notification_panel settings
- Remove project name setting (#278)

### Fixed

- Don't terminate connection on ignorable LSP messages
- Ensure that buffer is up to date after undo (zed#51037)
- Fix edited multibuffers not saving on focus change when `autosave` setting is set to `on_focus_change`
- Fix environment variables failing to load when `nu` is the login shell
- Fix gitignore trying to watch the home directory
- Fix markdown block quote continuation highlighting (zed#51465)
- Fix missing icon in GNOME overview. (#198) by @topas-rec
- Fix opening closed projects randomly when Gram restarts (zed#50961)
- Fix rewrapping with an empty selection (zed#51742)
- Fix scrollbar breaking when UI font size changes (zed#45099)
- Fix unknown capture warning for TOML files
- Fix watcher cleanup for recreated directories (zed#50412)
- Handle FS rescan events (zed#51208)
- Handle symlinked settings files in watcher
- Include file size in DiskState to fix stale buffer reload (zed#48691)
- Open named in-memory databases as SQLite URIs (zed#50967)
- Fix crash with Unicode chars whose lowercase expands to multiple codepoints (zed#52989)
- Fix an ordering problem that led to invalid edits in display map sync (zed#52930)
- Fix out-of-bounds indexing when diff bases contain CRLF (zed#52605)
- Fix alt keybindings capturing altgr input (Mac) (#206)
- Minor fixes to the build scripts to allow a clean build on Chimera Linux (#205) by @jmc
- Fix dollar sign rendering in Markdown preview (zed#50440)
- Prevent stack overflows in Markdown parsing (zed#51637)
- Fix Markdown preview not re-rendering on external change (zed#50583)
- Fix table wrapping behaviour in Markdown preview (zed#50839)
- Make non-zero number fields editable in settings (#232)
- fix: .gitattributes stats, update langs case by @koru (#257)
- Fix file rename on FUSE-based filesystems (#269) by @mxw
- Fix history navigation in multibuffers (#275)
- Fix incorrect window size on X11 (#259)
- Improved Supertab completion
- Fix application menus hover behavior (#276) by @sir_nacnud
- Fix tab completion in file finder (#270)
- Don't install compilation artifacts when installing extension (#273)
- Compile extension in release mode (#273)
- Update vscode-eslint to 3.0.24 and fix ESLint 8-10 (#227) (zed#52886)

### Changed

- Modified default settings to be more sensible (#184)
- Update to Rust 1.95
- Update and clean up dependencies
- Enable panel icons if default and icon theme is changed (#172)
- Add Arch package build
- Upgrade cosmic-text to 0.18.2 (#199) by @esotericwitch
- Use SSH nicknames in display names (zed#53103)
- Bump tree-sitter for fix to wasm loading of grammars w/ reserved words (zed#52856)
- Update .gitattributes (#226) by @ivankopylov6603
- Make status_bar.show a non-experimental setting
- Rework vim/helix mode indicator

## [1.2.1] - 2026-03-26

### Fixed

- Make sure to update Cargo.lock before tagging
- docs: Adopt the Zig project code of conduct policy on LLM use
- docs: Add toad to README
- docs: Add explanation for the name to README (#155)
- docs: Fix markdown link in README

## [1.2.0] - 2026-03-24

### Added

- Placeholder in empty pane when no file is open (#148)

### Fixed

- Use new icon format on Mac OS Tahoe (#146) by @kramo
- Fix: Use CompositeAlphaMode::Auto as fallback instead of ::Opaque (#149)
- Fix: handle wgpu surface recreation better (#149)
- Fix: Check for wasm-[component]-ld when detecting clang for WASI (#64)
- docs: Update installation instructions for Linux (#159) by @theDoctor
- docs: Fix a bunch of broken documentation links

### Changed

- Don't load defaults for Minimal base keymap (#151) by @tjk
- Increase contrast of selected item in Prompt window (#156) by @tjk
- Remove (untested) Snap support from script folder

## [1.1.0] - 2026-03-21

### Added

- Add AUR installation instructions to README (#38) by @nerdyslacker
- Add Gleam theme by Danielle Maywood
- Add Minimal base keymap for use with vim/helix keys (#144) by @tjk
- Add XML language server lemminx (#121) by @theDoctor
- Add XML treesitter support (#116) by @theDoctor
- Add appimage build option to bundle-linux (#114) by @theDoctor
- Add bash LSP (#132) by @theDoctor
- Add binary aur install instructions to README (#53) by @bananas
- Add default empty rustfmt config (#92) by @fzzr
- Add helix :reflow command to vim/helix modes
- Add new app icons by @kramo (#49) (#134)
- Add perl-Time-Piece to linux script (#1) by @LHolten
- Add remove trailing whitespace action (#81)
- Add subpixel text rendering, switch from blade to wgpu (#103) by @selfisekai
- Add zlib-ng-compat-static as dependency for Fedora build (#62) by @voidedgin
- Build and package for windows (#74) by @sparx
- Built in language support for Nix (#59)
- Built in language support for OpenTofu (#33) by @theDoctor
- Improved feedback when extension installation fails (#37)
- Options to adjust client side decoration rounding/shadow (#20)
- Support /path/to/file.txt(ln, col) style paths in terminal (#75)

### Fixed

- Bind vim/helix keys even if base keymap is None (#117)
- Cleanup remaining screen-capture code (#80) by @selfisekai
- Don't install WASI SDK if compatible clang is found (#41)
- Don't panic on failing to send an error notification (#136) by @selfisekai
- Enable the uninstall extension button (#32)
- Extend icon theme docs with enable settings (#6) by @Petrosz007
- Fetch newest release automatically (#83) by @scadu
- Fix LSP github download logic for pre-release (#67) by @Petrosz007
- Fix Supertab not performing word completion
- Fix `block_comment` and `documentation_comment` for Rust (#24) by @fzzr
- Fix download of tofu-ls for opentofu lsp support (#122)
- Fix empty project panel "or" divider (#109)
- Fix extensions being installed to tmp dir (#26)
- Fix menubar keyboard navigation (#119) by @aylamz
- Fix superhtml LSP (#56) by @Petrosz007
- Fix the lua-language-server asset name when downloading from github (#78)
- Fixed package name for cmake for Gentoo packages (#28) by @stepanov
- Fixes compilation issue on aarch64-linux (#61) by @voidedgin
- Make UI and Buffer Font match. (#69) by @voidedgin
- Modify single instance port numbers to not clash with Zed (#10)
- Remove more old sign-in code (#123) by @tjk
- Remove unused credentials code (#110)
- Set correct target for extension grammar (#84) by @selfisekai
- Use system clang, if it supports the wasm target (#64) by @selfisekai
- docs: Documentation around installing extensions. (#23) by @edwardloveall
- docs: Fix a few documentation links (#125) by @tjk
- docs: Fixed typos in README (#19) by @GulfSugar
- docs: Highlight that Rust in required to install some extensions (#34) by @ash-sykes
- docs: Remove mention of old feedback window (#126) by @tjk
- docs: Wasm is not an acronym

### Changed

- Move GLSL extension into [separate
  repo](https://codeberg.org/GramEditor/glsl-extension)
- Move Protobuf extension into [separate
  repo](https://codeberg.org/GramEditor/proto-extension)
- Allow for CARGO_TARGET_DIR in install.sh (#118) by @tjk
- Bump crash-handler to 0.7 (#39) by @selfisekai
- Update wild to 0.8.0 (#63) by @voidedgin
- (see [codeberg.org/GramEditor/gram](https://codeberg.org/GramEditor/gram) for
  complete list of changes)

## [1.0.0] - 2026-03-01

### Added

- First release
- Website: <https://gram.liten.app>
