# Change History

## 2026-07-15
- Added custom agent `project-context-keeper.agent.md` in `.github/agents/`.
- Added persistent context files:
  - `.github/project-state.md`
  - `.github/implementation-plan.md`
  - `.github/change-history.md`
- Purpose: preserve project context across chat sessions and improve handoff quality.

### Verification
- Files created successfully.

## 2026-07-15 (SysMon applet stabilization)
- Fixed compile error in applet update loop by replacing unsupported blocking task API.
- Implemented compact single-row panel rendering with SVG icons and metric values.
- Added icon assets:
  - `data/icons/cpu.svg`
  - `data/icons/gpu.svg`
  - `data/icons/memory-stick.svg`
- Added optional debug logging in applet via `SYSMON_DEBUG=1`.
- Added applet autosizing for top bar content.
- Updated `data/com.juancjr.SysMon.desktop`:
  - absolute `Exec`/`TryExec` path to `/home/juanc_jr_99/.cargo/bin/sysmon-applet`
  - added `X-CosmicShrinkable=true`
  - added `X-OverflowPriority=10`
- Enabled `libcosmic` feature `applet-token` in `Cargo.toml`.

### Verification
- `cargo build --bin sysmon-applet --release` finished successfully.
- `cargo install --path . --bin sysmon-applet --force` completed.
- process list confirms panel spawns `/home/juanc_jr_99/.cargo/bin/sysmon-applet`.

### Remaining risk
- Visual placement in panel can still fail if applet is duplicated or panel wing has insufficient space.