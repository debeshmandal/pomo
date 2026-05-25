# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Development
cargo run             # build and launch the app
cargo check           # fast type/borrow check (no binary produced)
cargo build           # debug build
cargo build --release # optimised release binary → target/release/pomo

# Quality
cargo clippy          # lints
cargo fmt             # auto-format
cargo test            # run tests (none yet)
```

First build is slow (~2 min) because eframe pulls ~400 crates. Subsequent builds are fast.

On Linux, eframe requires system packages for windowing. Install if the build fails:
```bash
sudo apt install libxkbcommon-dev libwayland-dev libx11-dev  # Debian/Ubuntu
```

## Architecture

Two-file app — no framework beyond eframe:

- **`src/main.rs`** — sets the fixed 340×500 window via `egui::ViewportBuilder`, disables the console window on Windows release builds (`windows_subsystem = "windows"`), and calls `eframe::run_native`.
- **`src/app.rs`** — contains everything else: the `Mode` enum, `PomodoroApp` struct, timer logic, and the egui UI.

### Timer state machine

`PomodoroApp` holds:
- `mode: Mode` — current phase (`Work` / `ShortBreak` / `LongBreak`)
- `remaining: Duration` — time left in the current phase
- `running: bool` — whether the clock is ticking
- `last_tick: Option<Instant>` — timestamp of the last `update()` call while running

`tick()` is called at the top of every `update()`. It computes `last.elapsed()`, subtracts from `remaining` with `saturating_sub`, and self-stops when `remaining` reaches zero. The `Instant` is replaced each tick (not accumulated), so pausing and resuming is exact with no drift.

`ctx.request_repaint()` is called only when `running == true`, so the app is idle (no CPU) when paused.

### UI

egui is an immediate-mode GUI: the entire UI is re-declared on every `update()` call. There is no retained widget state — all state lives in `PomodoroApp`.

The themed background (`Mode::color`) is applied by cloning `ctx.style()`, patching `visuals.panel_fill`, and calling `ctx.set_style()` each frame.

### Default durations

| Mode         | Duration |
|--------------|----------|
| Work         | 25 min   |
| Short Break  | 5 min    |
| Long Break   | 15 min   |

Durations are defined in `Mode::duration()` in `src/app.rs`.
