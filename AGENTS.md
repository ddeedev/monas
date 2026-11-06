# AGENTS.md

Guide for AI agents working in the **monas** repository.

## Project overview

**monas** is an Arc-like, macOS-first **web browser** written in Rust using the [GPUI](https://www.gpui.rs/) UI framework. It is in early development (v0.1.0).

> **Motto:** a crate boundary earns its cost when it turns a rule you care about into a compiler error. Here the rule is: widgets don't know the domain; feature views do; the app crate is the composition root. Per-crate READMEs (`crates/component/README.md`, `crates/ui/README.md`) document the rule for contributors.

> **Current focus:** this project began as "Silly", a REST API client with Arc-browser-style UX. The author's current focus is building the **browser** — that is where all active work happens. The REST-client feature may return later, so its types (`crates/context/src/api/`, `TabData::ApiRequest`) are kept but dormant. `ROADMAP.md` predates the pivot; treat it as historical.

The codebase is a Rust workspace of eight crates under `crates/`. The author uses a hobby-style workflow with `main`, `develop`.

## AI agent policy: mentor mode by default

This repository is a deliberate learning project. Contributors write their own code — agents working here must act as **mentors, not implementers**. Do not vibe code this project.

Unless the human explicitly opts out in the current session, an agent MUST:

1. **Never write or edit source code** (`crates/`, `page/`, `Cargo.toml`, CI). Reading is always allowed.
2. **Never paste drop-in implementations** of what the human is building. Guide with progressive hints instead: concept → direction (point to specific files/types) → approach in prose → small skeleton with `// you fill this in`. Escalate only when they remain stuck.
3. **Explain the why** behind Rust/GPUI idioms (ownership, `Entity`/`Context`, render cycle, actions, focus), and always hand the work back with a concrete next step and how to verify it (`cargo check`, `cargo run`, expected behavior).

**Meta-file exception ("ignore files"):** `AGENTS.md`, `README.md`, and skill files (`.claude/skills/`, `.agents/skills/`, `.opencode/skills/`) may be created or edited by an agent, but only when the human explicitly requests it in the current session.

**Opt-out:** only if the human explicitly confirms they want implementation (e.g. "leave mentor mode, write it for me") may an agent write code, and only for that task.

The full protocol lives in the mentor skill: [`.claude/skills/mentor/SKILL.md`](.claude/skills/mentor/SKILL.md). It is mirrored (hardlinked) into `.agents/skills/mentor/` and `.opencode/skills/mentor/` — one write updates all copies. Load and follow it.

Standing **design decisions** (intentional behavior, never flag as bugs) live in the design-decisions skill: [`.claude/skills/design-decisions/SKILL.md`](.claude/skills/design-decisions/SKILL.md). Consult it before reviewing or flagging findings.

## Repository structure

```text
/
├── Cargo.toml              # Workspace manifest (default member: crates/app)
├── Cargo.lock
├── crates/                 # Rust workspace members
│   ├── app/                # Application binary (`monas`)
│   │   └── src/
│   │       ├── main.rs     # Entrypoint → AppRunner::run()
│   │       ├── app.rs      # AppRunner: application, window, startup wiring
│   │       ├── menu.rs     # macOS application menu (titled "Monas"; View menu wires ToggleSidebar)
│   │       └── view/
│   │           ├── mod.rs            # Mock space data (hardcoded in code)
│   │           ├── main_content.rs   # MainContent: root view, spaces, actions
│   │           └── space_content.rs  # SpaceContent: per-space entity bundle
│   ├── action/             # Actions (ToggleSidebar, Quit, CloseTab, ResetSidebar,
│   │                       # OpenSearchPopup, SwitchSpace) + key bindings
│   ├── assets/             # Embedded assets (rust_embed SVG icons, fonts)
│   │   └── src/assets.rs   # Assets AssetSource + FONT_SUPPORT + load_fonts_asset
│   ├── component/          # Homegrown GPUI widgets + theme (domain-agnostic — see crates/component/README.md)
│   │   └── src/
│   │       ├── component.rs        # Crate root
│   │       ├── theme/mod.rs        # Theme, ActiveTheme, init() (pink preset)
│   │       ├── theme/theme_color.rs
│   │       ├── tile.rs, text_box.rs, list_row.rs, favicon.rs
│   │       ├── button.rs          # Button widget (focusable by default, keyed-state focus handles)
│   │       ├── window_control.rs  # WindowControl: custom traffic lights (close/minimize/zoom)
│   ├── context/            # Domain model (no GPUI dependency)
│   │   └── src/
│   │       ├── context.rs          # Crate re-exports
│   │       ├── node.rs             # Node / NodeId tree (Folder | Tab)
│   │       ├── tabs.rs             # TabData (Browser | ApiRequest)
│   │       ├── browser/space.rs    # SpaceContext, ProfileContext, SidebarContext
│   │       ├── browser/tab.rs      # BrowserData, favicon_for (Google s2 service)
│   │       ├── api/tab.rs          # ApiRequestData (dormant REST client, may return)
│   │       └── config/             # TOML Config trait (currently unwired/dead)
│   ├── input/               # Custom text input widget
│   │   └── src/input.rs     # TextInput entity, TextElement, EntityInputHandler
│   │                        # (adapted from GPUI's text_input example)
│   ├── setting/            # User settings crate (placeholder, currently dead code)
│   │   └── src/setting.rs
│   └── ui/                  # Larger GPUI views + platform helpers (feature views — see crates/ui/README.md)
│       └── src/
│           ├── ui.rs
│           ├── sidebar/mod.rs       # SidebarView: docked/floating state machine
│           ├── sidebar/app_sidebar.rs # AppSidebar: RenderOnce row rendering
│           ├── sidebar/event/mod.rs
│           ├── search/popup.rs      # Popup: cmd-t search overlay
│           ├── search/search_bar.rs
│           └── platform.rs          # macOS traffic-light hiding (objc2)
├── assets/                 # Assets referenced from crates/assets
│   ├── icons/              # SVG icons loaded via svg().path("icons/...")
│   ├── fonts/              # Bundled TTF fonts (6 shipped, 4 in FONT_SUPPORT)
│   └── mock/               # Mock TOML files (NOT currently embedded — see gotchas)
├── .github/workflows/      # deploy.yml (release) — currently fully commented out
├── .claude/skills/         # mentor/ + repo-scan/ + design-decisions/ skills (hardlinked into .agents/ and .opencode/)
├── .cursor/rules/          # Cursor rule deferring to the mentor-mode policy above
├── CLAUDE.md               # Imports this file for Claude Code
├── README.md               # Basic run instructions (skeletal)
├── NOTE.md                 # Author note
└── ROADMAP.md              # Pre-pivot roadmap (historical)
```

## Essential commands

### Rust application

The workspace default member is `crates/app`, so `cargo run` and `cargo build` target the browser app by default. The binary is named `monas`.

```bash
# Run the app locally (requires GPUI platform prerequisites; see README.md)
cargo run

# Build release binary
cargo build --release --locked
```

There are very few tests (one cargo-new template leftover in `crates/setting`, plus a WIP test-harness stub in `crates/component/src/button.rs` with no tests yet), no Makefile, and no custom build scripts.

## Architecture and control flow

### Entrypoint

`crates/app/src/main.rs` delegates everything to `app::AppRunner::run()`.

### Application setup (`crates/app/src/app.rs`)

`AppRunner::run()` performs the following:

1. Builds the `Application` via `gpui_platform::current_platform(false)` + `Application::new_inaccessible`, installing a `reqwest_client::ReqwestClient` HTTP client (user agent `monas/0.1`).
2. Registers the embedded asset source (`assets::Assets`).
3. Loads bundled fonts (`assets::load_fonts_asset`).
4. Initializes the theme (`component::theme::init` — currently the `pink()` preset).
5. Registers key bindings: `action::register_keybind` (global + `main_view` scope), `input::TextInput::bind_keys` (`TextInput` scope), `ui::search::popup::Popup::bind_keys` (`Popup` scope).
6. Initializes the macOS application menu (`menu::init_app_menu`).
7. Registers the `Quit` action handler; quits when the last window closes.
8. Opens a normal GPUI window with a transparent titlebar and traffic lights positioned at (12, 12), containing `MainContent`.

### View hierarchy

```text
Application
 └─ Window (transparent titlebar)
     └─ MainContent (crates/app/src/view/main_content.rs)  [.key_context("main_view")]
         ├─ Vec<SpaceContent> (one per space)
         │   └─ SpaceContent (space_content.rs)
         │       ├─ Entity<SpaceContext>      # domain state
         │       ├─ Entity<SidebarView>      # sidebar UI + animation state
         │       └─ Entity<Popup>            # cmd-t search overlay
         ├─ current sidebar (rendered)
         ├─ main content area (card)         # mouse-move hides floating sidebar
         └─ current popup (absolute overlay)
```

`MainContent` is `Focusable` + `Global` (the `Global` impl is currently unused), tracks the 1-based `current_space`, and builds each space in `build_space`:

- Moves the `SidebarContext` out of the mock `SpaceContext` into its own entity and observes it (`cx.observe(...).detach()`), so sidebar data changes trigger re-render. (The `Entity<SpaceContext>` itself is observed too, but nothing ever updates it — wiring without a writer until persistence lands.)
- Creates the `TextInput` entities for the sidebar search field and the popup.
- Subscribes to `PopupEvent` — every event closes the popup; the `Open`/`Search` handlers are TODO stubs.
- **Focus safety net:** `render` re-focuses `self.focus_handle` whenever `contains_focused` fails, so scoped `main_view` keybindings survive any focused element being unmounted (hidden sidebar, space switch, popup close). This is the fix pattern for the orphaned-focus bug class: a focused element that leaves the tree drops the context stack to the dispatch-tree root, and scoped bindings stop matching until focus returns to a live node.

Focus rules to remember when adding UI here: elements with `track_focus` (incl. `Button`, which is focusable by default) steal focus on mousedown; parents render before children in a frame, and `contains_focused` reads the *last rendered* frame — so a parent's render-time focus steal beats a child that mounts and focuses in the same frame (modals must claim focus in their own `render` while visible).

### Actions and key bindings (`crates/action`)

Declared with `actions!(app, [ToggleSidebar, Quit, CloseTab, ResetSidebar, OpenSearchPopup])` plus the custom `SwitchSpace(usize)` action. Bindings:

- `cmd-q` → `Quit` (global)
- `cmd-w` → `CloseTab` in `main_view` (**no handler exists yet** — pressing it does nothing)
- `cmd-s` → `ToggleSidebar` in `main_view`
- `ctrl-0` → `ResetSidebar` (global)
- `ctrl-1/2/3` → `SwitchSpace(n)` (global)
- `cmd-t` → `OpenSearchPopup` (global)

Handlers are attached on `MainContent`'s `.key_context("main_view")` div via `cx.listener`. The `Popup` crate additionally binds `up`/`down`/`ctrl-p`/`ctrl-n`/`escape`/`cmd-t` in the `Popup` key context (`cmd-t → Dismiss` while the popup is focused, which toggles against the global open binding). The file also carries Arc-browser shortcut reference tables as comments.

### Sidebar state machine (`crates/ui/src/sidebar/`)

`SidebarView` (sidebar/mod.rs) owns two related states:

- **Docked**: open/closed with animated `width_anim` (16 ms timer lerp loop spawned via `cx.spawn`).
- **Floating**: appears when the docked sidebar is hidden and the cursor hovers the left edge; driven by `floating_visible` / `floating_progress` (same lerp pattern, duplicated).

Toggle rule (`SidebarView::toggle`): if the floating sidebar is visible while docked is hidden, toggling restores the docked sidebar immediately (no slide-out); otherwise it toggles docked-hidden and disables floating. `AppSidebar` (app_sidebar.rs) is the `RenderOnce` component that renders the custom traffic lights, spaces row, essential tiles, folders, and tab rows from the shared `SidebarContext` entity (it borrows via `read(cx)` — no whole-context clone per frame). `SidebarView` emits `SidebarEvent::Search` — currently with **no subscriber** (a commented-out subscribe stub in `main_content.rs` references a nonexistent `SidebarEvent::Toggle`).

### Search popup (`crates/ui/src/search/`)

`Popup` is a `cmd-t`-style overlay: filter-as-you-type over a static item list, keyboard navigation (`confirm`/`select_next`/`select_prev`/`dismiss` bound in the `Popup` key context), emits `PopupEvent::Dismiss` / `Open(BrowserData)` / `Search(SharedString)`. While visible, its `render` claims focus for its input (the modal-focus pattern that wins the parent-before-child render race). It has a `reset_after_dismiss` flag, currently hardcoded `false` at the only construction site.

### Text input (`crates/input`)

A custom single-line `TextInput` entity implementing `EntityInputHandler` plus a hand-written `TextElement` (custom GPUI element with its own layout/paint). Adapted from GPUI's `text_input` example; handles UTF-16 offsets, grapheme/word boundaries, clipboard, and scoped key bindings.

### Theme and widgets (`crates/component`)

`Theme` + `ThemeColor` presets (`pink()` is active via `init`, `dark_purple()` is unused), exposed through the `ActiveTheme` trait (`cx.theme()`). Widgets: `tile`, `text_box`, `list_row`, `favicon`, `button` (focusable, keyed-state focus handles), `window_control` (custom traffic lights). Some UI (popup, sidebar badges) still hardcodes colors instead of reading the theme — theme adoption is partial.

### Domain model (`crates/context`)

- **`SpaceContext` / `SpaceId`**: a browser space (name, 1-based number, profile id, sidebar).
- **`ProfileContext` / `ProfileId`**: profile identity (pseudonym, optional email).
- **`SidebarContext`**: node tree as `HashMap<NodeId, Node>` plus ordered `folder`, `essential`, `pinned` lists (`pinned` currently unused; `essential` is the renamed favorites/pinned-tabs row).
- **`Node` / `NodeId`**: tree node; `NodeData` is `Folder` (child ids + expand flag) or `Tab` (`TabData` + open flag).
- **`TabData`**: `Browser(BrowserData)` or `ApiRequest(ApiRequestData)` (legacy REST).
- **`BrowserData::favicon_for`** resolves favicons through `google.com/s2/favicons`.
- **`config/`**: TOML `Config` trait — currently dead code (never called; uses blocking `fs::read_to_string` and `CARGO_MANIFEST_DIR`, which won't work in an installed build).

### macOS platform integration (`crates/ui/src/platform.rs`)

Uses Objective-C runtime calls (`objc2`, `raw-window-handle`) to hide the window traffic-light buttons. Native lights are replaced by the custom `WindowControl` in the sidebar header; hiding them every render is a recorded design decision (window controls intentionally vanish with the sidebar — see design-decisions skill). There is no non-macOS stub — the app compiles on macOS only.

## Asset handling (`crates/assets`)

- `Assets` embeds `../../assets/icons/*.svg` and `../../assets/mock/**/*.md` via `rust_embed`. Note: the mock directory now contains TOML files, so the `**/*.md` pattern embeds nothing.
- Fonts are `include_bytes!`-ed and registered via `load_fonts_asset` at startup; `FONT_SUPPORT` lists registered names (4 of the 6 shipped TTFs).
- SVGs are referenced by path, e.g. `svg().path("icons/sidebar-left.svg")`.

## Code conventions

- Rust edition 2024.
- Workspace dependencies are declared in the root `Cargo.toml` and referenced with `*.workspace = true` in member crates.
- Import style: `use gpui::{..., prelude::*};` followed by workspace crate imports (`context::...`, `ui::...`, `component::...`).
- GPUI element chains are heavily indented and chained; keep this style.
- Colors as `rgb(0x...)`, sizes with `px(...)`.
- Components follow the `new()` + consuming setter pattern, then implement `Render` or `RenderOnce`.
- Debug element IDs: `#sidebar-hover-zone`, `#floating-sidebar`, `#sidebar-resize-handle`.

## Testing approach

Nearly zero coverage — the only test is a cargo-new template leftover (`crates/setting`), plus a WIP harness stub in `crates/component/src/button.rs`. Best first targets: pure functions in `crates/input/src/input.rs` (UTF-16 offsets, grapheme/word boundaries), sidebar state transitions, `sidebar/util.rs` `animating`, and `Popup::filtered`. UI changes are verified manually with `cargo run`.

## CI/CD

**No CI is currently active.** Both workflows under `.github/workflows/` are fully commented out:

- `deploy.yml` (macOS aarch64 release): intended to run on tags `v*` and package the binary. **Before re-enabling, fix the stale `silly` binary name — the binary is `monas`** (the tar step would fail).
- `page.yml` (Zola site → gh-pages): references the deleted `page/` directory; re-enable only if the site returns.

## Branches

- `main`: default branch on remote.
- `develop`: integration branch; Zola site deploys from here.
- Feature branches may exist locally (e.g. `vibe`).

## Important gotchas

1. **The current focus is the browser.** REST-client types in `crates/context/src/api/` are dormant, not dead — the feature may return later. Keep them intact.
2. **macOS-only build (recorded decision — see design-decisions skill).** `cmd_key!` and `platform_bindings` in `crates/input/src/input.rs` have both platform arms, but `option_key!` is still only defined under `#[cfg(target_os = "macos")]` while being used in the cfg-free `word_bindings`, and `platform.rs` has no non-macOS stub.
3. **Naming:** package/binary and app menu are `monas`/`Monas`; only older docs still say "Silly". Prefer `monas` for anything new.
4. **Mid-refactor debris:** unused `Global` impl, `_new_space`/`_with_space`, unsubscribed `SidebarEvent::Search` (plus a commented-out subscribe stub referencing a nonexistent `SidebarEvent::Toggle`), `CloseTab` with no handler, `dark_purple()` theme, `reset_after_dismiss` hardcoded `false`, ~16 unread `ThemeColor` fields. **Two known live bugs** (from the 2026-11-06 scan): `Popup::focus` registers a new `cx.observe` per call and is called every rendered frame while visible (unbounded observer growth), and `MainContent::render` registers `window.on_next_frame` every render (forces a continuous redraw loop).
5. **Per-render clones:** `AppSidebar` now borrows `SidebarContext` via `read(cx)` with targeted clones (folder list, node names) — the old whole-context deep clone is gone. Keep it that way.
6. **Mock data is hardcoded** in `crates/app/src/view/mod.rs`; the embedded-mock glob matches nothing; the `config/` module is dead. Persistence is entirely unwired.
7. **GPUI deps are git-pinned** to zed-industries/zed via `Cargo.lock`; `http_client` is declared as a workspace dependency but used by no crate.
8. **Keep the build warning-free** where possible; verify UI changes by running `cargo run`.
9. **The skill files are hardlinks**, not symlinks — each skill (`mentor`, `repo-scan`, `design-decisions`) is one inode shared across `.claude/`, `.agents/`, `.opencode/`; one write updates all three copies.
