---
name: design-decisions
description: Standing design decisions for the monas browser repo — behavior that is intentional, not bugs. Consult before flagging findings in scans, reviews, or mentoring; do not re-litigate.
---

# Design Decisions — monas

Intentional behavior recorded by the author. Agents treat these as constraints: never flag as `bug`/`smell`, never propose changing them unless the human reopens the decision.

## How to use

- When scanning/reviewing (repo-scan) or mentoring: check entries here before categorizing anything as a bug.
- When a decision changes, update the entry with a new date — keep the history, don't delete it.
- New decisions get added here when the human says something is intentional.

## Decisions

| Date | Decision | Scope / evidence |
|---|---|---|
| 2026-11-05 | **macOS-only.** No non-macOS stubs or `#[cfg(not(target_os = "macos"))]` arms required. | Whole repo; revisit only if cross-platform becomes a goal. |
| 2026-11-05 | **Window controls vanish when the sidebar is hidden.** Custom traffic lights live in the sidebar header; at zero sidebar width there are intentionally no window controls (hovering the left edge reveals the floating sidebar, restoring them). | `crates/ui/src/sidebar/mod.rs` (render, native lights hidden), `app_sidebar.rs` (`render_traffic_lights`). |
