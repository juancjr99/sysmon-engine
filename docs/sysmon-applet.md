# SysMon Applet Notes

## Current Status
- Applet compiles and installs correctly.
- Panel launch path is configured via absolute executable path.
- Runtime metrics collection is working.
- UI is rendered as a compact single row (CPU, GPU, RAM).

## Build And Install
From project root:

```bash
cargo build --bin sysmon-applet --release
cargo install --path . --bin sysmon-applet --force
```

## Desktop Entry
File: `data/com.juancjr.SysMon.desktop`

Key fields:
- `X-CosmicApplet=true`
- `X-CosmicAppletId=com.juancjr.SysMon`
- `Exec=/home/juanc_jr_99/.cargo/bin/sysmon-applet`
- `TryExec=/home/juanc_jr_99/.cargo/bin/sysmon-applet`
- `X-CosmicShrinkable=true`
- `X-OverflowPriority=10`

Install for current user:

```bash
mkdir -p ~/.local/share/applications
cp data/com.juancjr.SysMon.desktop ~/.local/share/applications/com.juancjr.SysMon.desktop
```

## Debugging
### Check that panel is spawning SysMon
```bash
pgrep -af "sysmon-applet|cosmic-panel"
```

### Filter logs for SysMon only
```bash
journalctl --user -f | rg -i "com.juancjr.SysMon|sysmon-applet"
```

### Enable applet metric logs
```bash
SYSMON_DEBUG=1 sysmon-applet
```

## Known Pitfalls
- If `Exec=sysmon-applet` is used, panel may not find it because user service PATH can exclude `~/.cargo/bin`.
- Repeated `killall cosmic-panel` can produce noisy unrelated applet errors and confuse diagnosis.
- If SysMon is added multiple times in panel config, visibility can be inconsistent due to overflow.

## Recommended Runtime Checks
1. Keep only one SysMon instance in panel layout.
2. Place it in a panel area with enough width.
3. Re-add the applet after desktop entry changes.
4. Rebuild and reinstall after code updates.
