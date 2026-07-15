# Project State

## Current Objective
- Define and stabilize sysmon applet and CLI behavior.

## What Works Now
- Rust workspace structure is present.
- Binaries exist under `src/bin/` (`applet.rs`, `cli.rs`).

## In Progress
- Release build for applet currently needs validation/fix.

## Known Issues Or Blockers
- Last known command `cargo build --bin sysmon-applet --release` exited with code 101.

## Last Updated
- 2026-07-15