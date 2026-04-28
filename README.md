# reaction-time-test

A terminal-based reaction time test written in Rust, inspired by [Human Benchmark](https://humanbenchmark.com/tests/reactiontime). No browser needed -- just your terminal.

## How it works

5 rounds. A red screen appears -- wait for it to turn green, then press any key or click as fast as you can. Your reaction time is measured in milliseconds.

| Phase | What you see | What to do |
|-------|-------------|------------|
| Red | "Wait for green..." | Don't press anything |
| Green | "CLICK!" | React as fast as possible |
| Blue | Your time + rating | Press any key for next round |
| Summary | All results + stats | `r` to restart, `q` to quit |

### Ratings

| Time | Rating |
|------|--------|
| < 150ms | Insane! |
| 150-200ms | Fast! |
| 200-250ms | Average |
| 250-350ms | Slow |
| > 350ms | Are you asleep? |

## Installation

### AUR

```bash
yay -S reaction-time-test
```

### From source

```bash
git clone https://github.com/fibsussy/reaction-time-test.git
cd reaction-time-test
cargo build --release
```

The binary will be at `target/release/reaction-time-test`.

### Cargo

```bash
cargo install --git https://github.com/fibsussy/reaction-time-test.git
```

## Usage

```bash
reaction-time-test
```

That's it. Supports keyboard input, mouse clicks, and the delay between rounds is randomized (1.5-5s) so you can't cheat by timing it.

### Controls

- **Any key / mouse click** -- react / advance
- **q** or **Esc** -- quit
- **Ctrl+C** -- quit from anywhere
- **r** -- restart (from summary screen)

## License

MIT
