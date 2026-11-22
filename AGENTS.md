# AGENTS.md

Interactive Rust CLI that restores Rue's CachyOS setup: installs packages, configures rustup,
creates a GitHub SSH key, then clones private config repos into `~/.config/<app>/`.

## Build and verify

- `cargo check` and `cargo clippy --all-targets -- -D warnings`; there are no tests.
- `cargo run` is the program: it prompts for confirmation, uses `sudo`/`paru`, deletes existing
  config dirs and clones from GitHub. Do not run it unless the task explicitly asks for a real
  restore.
- Edition 2024, zero dependencies. Pushing a tag matching `v[0-9]+.*` triggers
  `.github/workflows/release.yml`, which builds `x86_64-unknown-linux-musl` and publishes a release.
- `target/` is already tracked even though `.gitignore` now contains `/target`; cargo commands still
  dirty the worktree until `git rm -r --cached target` is committed. Never stage `target/`.

## Architecture

- `src/main.rs` is the whole flow: a linear sequence of yes/no prompts, with a retry loop around each
  step. To add a feature, add a module under `src/scripts/`, import it here, and append a prompt +
  loop in the same style.
- `src/scripts/installers/`: hardcoded package arrays. `pacman` runs through `sudo`, AUR through
  `paru`, Flatpak through `flatpak`.
- `src/scripts/git_scripts/`: one module per app. Each wipes the destination with `remove_files`,
  then calls `git_clone`. `git_clone` is `pub(super)`; reuse it instead of spawning git again.
- `src/util/`: `confirm_dialog` (defaults on empty/EOF) and `remove_files` (ignores `NotFound`).
- `src/scripts/script_error.rs`: the error enums. `main.rs` matches on each variant, so a new error
  kind also needs handling in the prompt loops.

## Ordering and gotchas

- Order matters: installs → rustup → niri (HTTPS) → git identity + SSH key → SSH-cloned configs
  (fish, fastfetch, nvim, alacritty, opencode). SSH clones fail before the key exists.
- Niri deliberately has both `configure_niri` (HTTPS, runs before SSH) and `configure_niri_with_ssh`.
- Most config repos are private and cloned with `git@github.com:Rue0612/...`. The opencode step clones
  `git@github.com:Rue0612/rue_opencode_config.git` over `~/.config/opencode/`; that config can
  contain secrets (`service.json`, tokens), so never print or commit its contents. Reading and
  editing `~/.config/opencode/service.json` is denied in the global opencode permissions; do not
  weaken that rule.
- Existing public names contain typos: `ComandFailed`, `configure_alacritty_with_shh`,
  `configure_fastfatch_with_ssh`, `install_flatpack_apps`, `GitShhScriptError`. Keep them unless you
  update every caller; do not fix one in isolation.
- Every destination is hardcoded to `$HOME/.config/<app>`; there is no `XDG_CONFIG_HOME` handling.
- `backup-noctalia-shell-config.json` is an unused leftover from removed Noctalia support; the code
  never reads it.

## Opencode packages

- `opencode` (TUI) and `opencode-desktop` (GUI) are official `extra` packages; the desktop package
  depends on the CLI. Both belong in the pacman list, not the AUR list.
- The opencode config repo's `.gitignore` now excludes `service.json`, but the file was already
  tracked, so `git rm --cached service.json` still needs to be committed there.

## Nvim config dependencies

The nvim repo (`Rue0612/kickstart.nvim`) needs these from the pacman step: `git`, `make`, `gcc`,
`ripgrep`, `fd`, `tree-sitter-cli`, `unzip`, `wl-clipboard`, a Nerd Font, plus `npm` (Mason installs
`markdownlint`) and `alacritty` (used by `<leader>tt`). `rust-analyzer`, `clippy` and `rustfmt` come
from the rustup step, not pacman.
