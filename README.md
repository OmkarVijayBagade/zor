# ZOR

A terminal-based TUI animation showcase built in Rust. Features a collection of smooth, visually satisfying animations rendered using `ratatui` and `crossterm`.

## Features

- Full-screen alternate buffer rendering
- Smooth 30 FPS animations with delta-time updates
- Proper terminal cleanup on exit
- Resize handling for all animations
- Keyboard & mouse navigation
- Modular, clean architecture

## Installation

```bash
git clone https://github.com/your-username/zor.git
cd zor
cargo run --release
```

## Usage

Run the application:
```bash
cargo run
```

### Controls

| Key         | Action                     |
|-------------|----------------------------|
| `1-9`       | Select & launch animation  |
| `↑` / `k`   | Navigate up                |
| `↓` / `j`   | Navigate down              |
| `Enter`     | Launch selected animation  |
| `q`         | Quit / Return to menu      |
| `Mouse`     | Click to select, scroll to navigate |

## Animations

| # | Name      | Description                                                                 |
|---|-----------|-----------------------------------------------------------------------------|
| 1 | Starfield | Stars falling with depth effect using `.`, `*`, `+` characters              |
| 2 | Matrix    | Falling character streams with trailing green gradient                      |
| 3 | Wave      | Composite sine wave visualization with smooth color mapping                 |
| 4 | Snake     | Auto-playing snake that hunts for food and grows                            |
| 5 | Fire      | Rising heat gradient effect with realistic flickering                       |
| 6 | Rain      | Diagonal rain with wind drift and splash particles                          |
| 7 | Swarm     | 150 particles orbiting a drifting center with depth-based rendering         |
| 8 | Circuit   | Slow-growing circuit board patterns using box-drawing characters            |
| 9 | Void      | Carving effect that expands empty space from a fully filled screen          |

## Project Structure

```
src/
├── main.rs              # Entry point, terminal setup, event loop
├── app.rs               # Application state & mode management
├── ui.rs                # Menu UI rendering
└── animations/
    ├── mod.rs           # Module declarations
    ├── animation_trait.rs # Core Animation trait
    ├── starfield.rs     # Falling stars animation
    ├── matrix.rs        # Matrix rain effect
    ├── wave.rs          # Sine wave visualization
    ├── snake.rs         # Auto-playing snake simulation
    ├── fire.rs          # Heat gradient fire effect
    ├── rain.rs          # Diagonal rain with splashes
    ├── swarm.rs         # Orbiting particle swarm
    ├── circuit.rs       # Circuit board pattern growth
    └── void.rs          # Screen carving void effect
```

## Dependencies

- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation
- [rand](https://crates.io/crates/rand) - Random number generation
- [clap](https://crates.io/crates/clap) - CLI argument parsing

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

---

Built with Rust & ratatui
