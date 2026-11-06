# component — design system crate

Homegrown GPUI widgets and the theme for **monas**. This crate is
**domain-agnostic by design**.

> **Project motto:** a crate boundary earns its cost when it turns a rule you
> care about into a compiler error. Here the rule is: widgets don't know the
> domain; feature views do; the app crate is the composition root.

## The rule

**Widgets don't know the domain.** This crate's `Cargo.toml` deliberately
declares _no_ dependency on `context` or `action` — a widget here physically
cannot import `SidebarContext`, `BrowserData`, or any app action. That is the
point: the layering is enforced by cargo, not by convention. Do not add those
dependencies to make a widget "just this once" convenient — the widget belongs
in `crates/ui` instead (see that crate's README).

## What belongs here

- Reusable widgets: `Button`, `Tile`, `TextBox`, `ListRow`, `Favicon`,
  `WindowControl`
- Theme: `Theme`, `ThemeColor`, `ActiveTheme` (`cx.theme()`)

**Test for admission:** if the file imports `context::` or `action::`, it is a
feature view → it belongs in `crates/ui`. If it only needs `gpui` (and at most
`input`), it belongs here.

## Layering

```text
app (composition root — wires everything, owns window/menu/mock data)
 └─ ui (feature views — may use context + action)
     └─ component (widgets + theme — may NOT use context/action)
```

Imports may only point downward in this chain. Never upward.
