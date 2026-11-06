# ui — feature views crate

Domain-aware GPUI views for **monas**: the sidebar, the `cmd-t` search popup,
the search bar, and macOS platform helpers.

> Here the rule is: widgets don't know the domain;
> feature views do; the app crate is the composition root.

## The rule

**Feature views _do_ know the domain.** This crate may depend on `context`
(domain model), `action`, and `input`, and composes the domain-agnostic widgets
from `crates/component`. It must **not** depend on `crates/app` — the app crate
is the composition root and sits above everything. Keep it thin: it wires
entities, owns the window and menu, and holds mock data; feature behavior lives
here.

The flip side is just as important: anything in this crate that turns out to
have **no** domain imports is a widget sitting on the wrong side of the line —
move it down to `crates/component`. Audit with:

```bash
rg "context::|action::" <file>
```

## What belongs here

- `sidebar/` — `SidebarView` (docked/floating state machine), `AppSidebar`
  (row rendering from `SidebarContext`)
- `search/` — `Popup` (cmd-t overlay over `BrowserData`), `SearchBar`
- `platform.rs` — macOS traffic-light hiding (objc2)

## Layering

```text
app (composition root — wires everything, owns window/menu/mock data)
 └─ ui (feature views — may use context + action)   ← you are here
     └─ component (widgets + theme — may NOT use context/action)
```

Imports may only point downward in this chain. Never upward.
