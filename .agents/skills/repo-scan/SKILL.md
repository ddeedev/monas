---
name: repo-scan
description: Use when the user asks to scan/review/audit the monas repo and suggest improvements or refactors. Produces a prioritized, file:line-cited report. Read-only on source code (mentor mode applies); may only edit ignore files (AGENTS.md, README.md, skill files) when explicitly asked.
---

# Repo Scan — Audit & Suggest

A repeatable audit workflow for the monas workspace. The output is a prioritized report the human acts on themselves — never a patch.

## Rules

- **Read-only on source.** Scanning is reading, which is always allowed; fixing is the human's job (see the mentor skill).
- **Verify every claim** against current code — no assumptions from memory or previous reports. Line numbers must be current.
- **Every finding needs `file:line`**, a category, and the *concept* needed to fix it (teach, don't just list).
- **Categorize** findings: `bug` / `smell` / `dead-code` / `design-gap` / `docs-stale`.
- **Do not re-explain** items already documented in AGENTS.md gotchas — just verify and one-line them.
- **Do not flag** the dormant REST types (`crates/context/src/api/`) for deletion — the feature may return.

## Workflow

1. **Ground truth first.** `git status --short` + `git log --oneline -8` (uncommitted work? recent pivots?). Read `AGENTS.md` for structure, conventions, and known gotchas.
2. **Build health.** `cargo check` — capture every warning/error. "Warning-free" is a repo value; warnings are findings.
3. **Verify known issues.** AGENTS.md gotchas + findings from previous scans. Each gets a status: FIXED / STILL PRESENT / CHANGED, with current `file:line`.
4. **Pattern sweep** (grep the workspace):
   - `unwrap()` / `expect()` / `panic!` in non-test code
   - dead code: compiler warnings, underscore-prefixed `pub` fns, unused workspace deps (root `Cargo.toml` vs member manifests)
   - `#[cfg(target_os)]` gaps (macros/calls used outside their guard)
   - `TODO` / `FIXME`
   - per-render clones: `.read(cx).clone()` in render paths
   - duplicate type definitions across crates
   - hardcoded colors (`0x…`) in views that bypass `cx.theme()`
   - events emitted with no subscriber; actions/keybindings with no handler
   - stale names (`silly` vs `monas`)
5. **Deep-read hot files.** Longest render functions, state machines (sidebar), custom elements (`TextElement`), domain model (`node.rs`, tree invariants), platform code (`platform.rs`), CI workflows, docs accuracy vs reality.
6. **Tests.** List every `#[test]`; note the best untested pure logic.
7. **Praise explicitly.** Name idiomatic patterns worth keeping — the human is learning; reinforcement is part of the job.

## Output format

1. **Status table** of previously-known items (one line each: item → status → `file:line`).
2. **New findings** grouped by category, each with `file:line`, one-sentence why, and the concept to fix it.
3. **Praiseworthy** patterns.
4. **Suggested order** (risk × effort), ending with a mentor hand-back: which item to start with, what to try, and how to verify (`cargo check`, `cargo run`, expected behavior).

Keep it concise: tables over prose. This repo's known hotspots: `crates/ui/src/sidebar/`, `crates/ui/src/search/popup.rs`, `crates/input/src/input.rs`, `crates/context/src/node.rs` (no mutation/removal API yet), `crates/component/src/theme/` (16+ unread `ThemeColor` fields), macOS-only cfg gaps.
