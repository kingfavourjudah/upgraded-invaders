# Invaders Game in Rust

A simple terminal-based space invaders game written in Rust using the `crossterm` and `rand` crates.

## How to Play

- **Move Left**: Left Arrow
- **Move Right**: Right Arrow
- **Shoot**: Spacebar
- **Quit**: Esc

## How to Run

1. Clone the repository.
2. Run the game with:

```bash
cargo run


---

### Summary

- **Modularity**: Breaking the code into `player.rs`, `bullet.rs`, and `enemy.rs` improves readability.
- **Main Loop**: The `main.rs` file coordinates updates and rendering.
- **Dependencies**: `crossterm` for terminal handling and `rand` for randomness.

This project is an improvement of the original code found at https://github.com/CleanCut/invaders. The code has been broken down into separate modules (player.rs, bullet.rs, and enemy.rs) for better readability and maintainability. The main.rs file coordinates updates and rendering, and the dependencies crossterm for terminal handling and rand for randomness have been utilized. This structure makes the game easier to maintain and extend! 🚀

This structure makes the game easier to maintain and extend! 🚀
