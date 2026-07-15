# Project State

## Current Objective
- Stabilize SysMon applet rendering in COSMIC top panel and keep metrics visible in a compact row.

## What Works Now
- `sysmon-applet` builds in release mode.
- Metrics backend works (validated with `cli` output).
- Custom SVG icons for CPU, GPU, and RAM are bundled under `data/icons/`.
- Panel now launches the applet from absolute path in desktop entry.

## In Progress
- Fine-tuning visibility in the top panel when panel space is tight.

## Known Issues Or Blockers
- Visual behavior in the panel can still be affected by panel overflow and duplicate applet placement.
- Aggressive manual panel restarts (`killall cosmic-panel`) generate noisy unrelated applet errors in logs.

## Last Updated
- 2026-07-15