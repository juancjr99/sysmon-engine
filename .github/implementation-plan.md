# Implementation Plan

## Active Plan
1. [done] Replace unsupported task API with compatible `cosmic::iced::Task::perform` flow.
2. [done] Add compact single-row applet UI with custom CPU/GPU/RAM SVG icons.
3. [done] Ensure panel can launch applet by using absolute `Exec` and `TryExec` path.
4. [done] Enable `libcosmic` feature `applet-token` for panel runtime compatibility.
5. [in-progress] Validate final visibility behavior in real panel layouts (single placement, no overflow).
6. [todo] Add optional compact/extended display mode switch (bar vs tooltip detail).
7. [done] Document deployment and troubleshooting steps.

## Notes
- Keep statuses in sync: `todo`, `in-progress`, `done`.
- Prefer small, verifiable steps.