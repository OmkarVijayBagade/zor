# ZOR

A terminal-based TUI animation showcase built in Rust. Features a collection of smooth, visually satisfying animations rendered using `ratatui` and `crossterm`.

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Platform-macOS%20%7C%20Linux-blue?style=for-the-badge" alt="Platform" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License" />
</p>

## Features

- Full-screen alternate buffer rendering
- Smooth 30 FPS animations with delta-time updates
- Proper terminal cleanup on exit
- Resize handling for all animations
- Keyboard-driven navigation with search & multi-digit selection
- Modular, clean architecture

## Installation

### Homebrew (macOS)

```bash
brew tap OmkarVijayBagade/zor
brew install zor
```

### GitHub Releases

Download pre-built binaries from the [Releases](https://github.com/OmkarVijayBagade/zor/releases) page:

```bash
# macOS Apple Silicon
curl -LO https://github.com/OmkarVijayBagade/zor/releases/latest/download/zor-macos-arm64
chmod +x zor-macos-arm64
sudo mv zor-macos-arm64 /usr/local/bin/zor

# macOS Intel
curl -LO https://github.com/OmkarVijayBagade/zor/releases/latest/download/zor-macos-x86_64
chmod +x zor-macos-x86_64
sudo mv zor-macos-x86_64 /usr/local/bin/zor

# Linux
curl -LO https://github.com/OmkarVijayBagade/zor/releases/latest/download/zor-linux-x86_64
chmod +x zor-linux-x86_64
sudo mv zor-linux-x86_64 /usr/local/bin/zor
```

### Build from Source

```bash
git clone https://github.com/OmkarVijayBagade/zor.git
cd zor
cargo install --path .
```

## Usage

Run the application:
```bash
zor
```

### Controls

| Key            | Action                                  |
|----------------|-----------------------------------------|
| `1-11`         | Jump to animation number (multi-digit)  |
| `↑` / `k`      | Navigate up                             |
| `↓` / `j`      | Navigate down                           |
| `Enter`        | Launch selected animation               |
| `/`            | Open search mode                        |
| `Esc`          | Exit search / cancel number input       |
| `q`            | Quit / Return to menu                   |

### Search Mode

Press `/` to filter animations by name. Type to filter in real-time, `Enter` to launch, `Esc` to exit.

### Number Selection

Type a number (e.g., `1`, `0`) to jump directly to that animation. The highlight moves as you type. Press `Enter` to launch.

## Animations

| #  | Name      | Description                                                                 |
|----|-----------|-----------------------------------------------------------------------------|
| 1  | Starfield | Stars falling with depth effect using `.`, `*`, `+` characters              |
| 2  | Matrix    | Falling character streams with trailing green gradient                      |
| 3  | Wave      | Composite sine wave visualization with smooth color mapping                 |
| 4  | Snake     | Auto-playing snake that hunts for food and grows                            |
| 5  | Fire      | Rising heat gradient effect with realistic flickering                       |
| 6  | Rain      | Diagonal rain with wind drift and splash particles                          |
| 7  | Swarm     | 150 particles orbiting a drifting center with depth-based rendering         |
| 8  | Circuit   | Slow-growing circuit board patterns using box-drawing characters            |
| 9  | Void      | Carving effect that expands empty space from a fully filled screen          |
| 10 | Flux      | Fast diagonal pipe growth that fills 40% of screen before resetting         |
| 11 | Drift     | Flow field particles following organic invisible currents                   |

## Project Structure

```
src/
├── main.rs                  # Entry point, terminal setup, event loop
├── app.rs                   # Application state, input modes, menu logic
├── ui.rs                    # Menu UI rendering (header, list, search, footer)
└── animations/
    ├── mod.rs               # Module declarations
    ├── animation_trait.rs   # Core Animation trait
    ├── starfield.rs         # Falling stars animation
    ├── matrix.rs            # Matrix rain effect
    ├── wave.rs              # Sine wave visualization
    ├── snake.rs             # Auto-playing snake simulation
    ├── fire.rs              # Heat gradient fire effect
    ├── rain.rs              # Diagonal rain with splashes
    ├── swarm.rs             # Orbiting particle swarm
    ├── circuit.rs           # Circuit board pattern growth
    ├── void.rs              # Screen carving void effect
    ├── flux.rs              # Diagonal pipe growth
    └── drift.rs             # Flow field particles
```

## Dependencies

- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation
- [rand](https://crates.io/crates/rand) - Random number generation
- [clap](https://crates.io/crates/clap) - CLI argument parsing

## CI/CD

This project uses GitHub Actions for automated releases. Pushing a tag like `v0.1.0` triggers:

- Build for Linux (x86_64), macOS Intel, and macOS ARM
- Automatic GitHub Release creation
- Binary assets uploaded to the release

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

---

Built with Rust & ratatui
