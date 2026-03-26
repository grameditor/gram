# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
