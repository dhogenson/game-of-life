# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a fully functional graphical implementation of Conway's Game of Life in Rust with an interactive editor. The game includes:

- Complete Game of Life simulation following Conway's standard rules
- Interactive mouse-based cell editor
- Real-time generation tracking and population display
- Graphical rendering using Piston game engine
- Clean cell visualization (black = alive, white = dead)
- Mouse controls for cell manipulation and keyboard controls for simulation
- Auto-advance mode for continuous simulation

**Status**: Core functionality complete. Game logic, rendering, and mouse controls are all implemented. UI text display is a placeholder for future enhancement.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the project
cargo run

# Build with optimizations (recommended for smoother performance)
cargo build --release

# Run release build
cargo run --release

# Check code without building
cargo check

# Run tests (when implemented)
cargo test

# Run specific test
cargo test test_name

# Format code (always run before committing)
cargo fmt

# Run linter (checks for common mistakes and style issues)
cargo clippy

# Run linter with all warnings
cargo clippy -- -W clippy::all
```

## Controls

When running the application:

- **Left Mouse Click**: Toggle cell state (alive/dead) at mouse position
- **Right Arrow (→)**: Advance simulation by one generation
- **F (Hold)**: Auto-advance mode - continuously advances simulation at 20 ticks/second
- **R**: Reset/clear the board (all cells become dead, generation resets to 0)
- **ESC**: Quit the application

## Architecture

The codebase follows a modular structure with clear separation of concerns:

- **`src/main.rs`**: Entry point, window management, event loop, mouse handling, and rendering
- **`src/board.rs`**: Board struct with Game of Life logic

### Main Module (src/main.rs)

Entry point that:

- Initializes a 50x50 board
- Creates a Piston window (1000x1000 pixels dynamically calculated from board size)
- Sets up window with VSync enabled and ESC key to exit
- Runs the main event loop at 60 FPS with Piston's event system
- Handles keyboard events for simulation control
- Handles mouse events for cell editing
- Tracks pressed keys using HashSet for continuous actions
- Implements auto-advance mode (F key held) with configurable tick rate
- Renders the game board on each frame using `draw_2d()`
- Tracks generation count (u128) and mouse position

**Configuration Constants**:

- `CELL_SIZE`: 20.0 pixels per cell
- `BOARD_WIDTH`: 50 cells
- `BOARD_HEIGHT`: 50 cells
- `FPS`: 60 frames per second
- `AUTO_TICK_INTERVAL_MS`: 50 milliseconds between auto-ticks (20 ticks/second when F is held)

**Rendering Functions**:

- `draw_ui()`: Placeholder for UI text (controls, generation, population) - currently unimplemented (src/main.rs:126-128)
- `draw_board()`: Renders all cells as rectangles with color coding

**Platform-Specific Features**:

- Windows release builds hide the console window using `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`

**Key Features**:

- **Mouse Tracking**: Continuously tracks mouse position and converts to board coordinates
- **Pressed Keys Tracking**: Uses HashSet to track currently pressed keys for continuous actions
- **Auto-Advance Mode**: When F key is held, automatically advances simulation at 20 ticks/second
- **Timing System**: Uses `Instant` and `Duration` to control auto-advance tick rate independently of FPS

### Board Module (src/board.rs)

The `Board` struct represents the game grid:

- **Fields**:

  - `size_x`, `size_y`: Grid dimensions (i8)
  - `board: Vec<Vec<i8>>`: 2D grid storing cell states (0 = dead, 1 = alive)
  - `population: u64`: Current count of living cells

- **Methods**:
  - `new(size_x, size_y)`: Constructor that creates a board with specified dimensions
  - `make_board(size_x, size_y)`: Static method to initialize an empty grid
  - `player_toggle_cell(board_x, board_y)`: Toggles cell state at given coordinates and updates population
  - `clear_board()`: Resets all cells to dead (0) and population to 0
  - `tick()`: Advances simulation by one generation following Conway's rules
  - `get_neighbour_count(x, y)`: Counts living neighbors for a given cell (used by tick)

## Dependencies

Listed in `Cargo.toml`:

- **piston_window** (0.130.0): 2D graphics and window management library
  - Used for: Window creation, event handling, 2D rendering, keyboard/mouse input
  - Provides: Graphics context, rectangle drawing, frame rendering, VSync

**Standard Library Usage**:

- **std::time**: Duration and Instant for timing auto-advance ticks
- **std::collections::HashSet**: Tracking currently pressed keys for continuous actions

**Edition**: Rust 2024 (latest edition)

## Game of Life Rules

The implementation follows Conway's standard rules (implemented in `Board::tick()`):

- Any live cell with 2-3 live neighbors survives
- Any dead cell with exactly 3 live neighbors becomes alive
- All other live cells die (underpopulation or overpopulation)
- All other dead cells stay dead

## Rendering Details

### Visual Design

- **Window Size**: 1000x1000 pixels (dynamically calculated from board size)
- **Cell Size**: 20x20 pixels with 1px gap between cells
- **Background Color**: White [1.0, 1.0, 1.0, 1.0]
- **Living Cells**: Black [0.1, 0.1, 0.1, 1.0]
- **Dead Cells**: White [1.0, 1.0, 1.0, 1.0]

### Rendering Pipeline

1. Clear screen to white background
2. Call `draw_ui()` for controls/stats (currently a placeholder)
3. Call `draw_board()` which:
   - Iterates through all cells (50x50 = 2500 cells)
   - Draws each cell as a rectangle with appropriate color (black for alive, white for dead)

## Coding Conventions

When working with this codebase:

### General Rust Practices

- **Use `cargo fmt`** before committing to maintain consistent formatting
- **Run `cargo clippy`** to catch common mistakes and non-idiomatic code
- **Prefer `i8`** for coordinate types to match existing pattern (allows negative intermediate calculations)
- **Use `u64`** for counters like population that should never be negative
- **Use `u128`** for generation count to handle very long-running simulations
- **Derive `Clone`** for structs that need to be passed around (Board, Player)
- **Use `pub` fields** for simple data structures where encapsulation isn't critical

### Code Style

- **Explicit return types**: Always specify return types for functions (e.g., `-> Self`, `-> i8`)
- **Early returns**: Use guard clauses for boundary checks
- **Explicit mutability**: Only mark variables as `mut` when they need to change
- **Clone when necessary**: The codebase uses `.clone()` for Board and Player to avoid borrow checker complexity
- **Comprehensive comments**: Include doc comments (`///`) for all public functions and structs

### Naming Conventions

- **Snake_case** for functions and variables: `get_neighbour_count`, `next_board`, `cell_size`
- **PascalCase** for types: `Board`, `Player`
- **SCREAMING_SNAKE_CASE** for constants: `CELL_SIZE`, `BOARD_WIDTH`, `BOARD_HEIGHT`
- **Descriptive names**: `neighbour_count` not `nc`, `current_state` not `state`
- **Coordinate conventions**: `x` is horizontal, `y` is vertical; `size_x` is width, `size_y` is height

### Module Organization

- Keep related functionality together in the same module
- Use `pub` only for items that need to be accessed from other modules
- Import external crates at the top of files
- Import local modules with `use crate::module_name`
- Group imports by category (external crates, then local modules)

### Piston-Specific Patterns

- **Generic graphics**: Use `<G: Graphics>` for rendering functions to work with any graphics backend
- **Context and transform**: Always use `context.transform` for positioning rectangles
- **Color format**: RGBA as `[f64; 4]` with values 0.0-1.0
- **Event handling**:
  - Match on `Button::Keyboard(key)` for keyboard input
  - Match on `Button::Mouse(button)` for mouse button input
  - Use `event.mouse_cursor_args()` to get mouse position
  - Use `event.press_args()` and `event.release_args()` for button events
- **Drawing**: Call `window.draw_2d(&event, |context, graphics, _device| { ... })`
- **Event loop**: Use `Events::new(EventSettings::new().max_fps(fps))` to configure FPS

## Testing Strategy

Currently, no tests are implemented. When adding tests:

### Unit Tests

- Add `#[cfg(test)]` modules at the bottom of each source file
- Test `Board::get_neighbour_count()` with various cell configurations
- Test `Board::tick()` with known Game of Life patterns (blinkers, gliders, still lifes)
- Test `Board::clear_board()` to ensure it resets state correctly
- Test `Board::player_toggle_cell()` with various coordinates
- Test population counting accuracy

### Integration Tests

- Create `tests/` directory for integration tests
- Test full game loop scenarios
- Test pattern evolution over multiple generations
- Verify known patterns (gliders should glide, blinkers should blink)

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbour_count_empty_board() {
        let board = Board::new(10, 10);
        assert_eq!(board.get_neighbour_count(5, 5), 0);
    }

    #[test]
    fn test_clear_board() {
        let mut board = Board::new(10, 10);
        board.player_toggle_cell(0, 0);
        assert_eq!(board.population, 1);
        board.clear_board();
        assert_eq!(board.population, 0);
    }

    #[test]
    fn test_blinker_pattern() {
        // Test that a blinker oscillates correctly
    }
}
```

## Contribution Guidelines

### Before Submitting Changes

1. **Format code**: Run `cargo fmt`
2. **Check for errors**: Run `cargo clippy`
3. **Build successfully**: Run `cargo build`
4. **Test manually**: Run `cargo run` and verify functionality
5. **Run tests**: Run `cargo test`

### Making Changes

- **Small, focused commits**: Each commit should do one thing
- **Descriptive commit messages**: Explain what and why, not just what
- **Maintain existing patterns**: Follow the established code style
- **Update this file**: If you change architecture or add features, update CLAUDE.md

### Performance Considerations

- The game uses `clone()` liberally for simplicity; consider refactoring to use references for very large boards
- Piston handles rendering efficiently with VSync enabled and 60 FPS cap
- The 50x50 grid (2500 cells) renders smoothly on modern hardware
- Release builds (`--release`) are recommended for best performance
- Auto-advance mode runs at 20 ticks/second (50ms interval) when F key is held
- Board logic (tick) is fast; rendering is the primary bottleneck for very large boards

## Future Enhancements

Potential features to add:

### High Priority

- **Implement UI text display**: Complete the `draw_ui()` function to show controls, generation count, and population (src/main.rs:126-128)
- **Text rendering**: Integrate a text rendering solution (e.g., `piston_window::Glyphs`, `glyph_brush`)

### Feature Enhancements

- **Preset patterns**: Load famous Game of Life patterns (glider guns, spaceships, pulsars, etc.)
- **Save/Load**: Serialize board state to files (JSON, binary, or RLE format)
- **Configurable board size**: Command-line arguments for dimensions
- **Configurable auto-advance speed**: Make tick interval adjustable via keyboard controls
- **Pause/Resume**: Toggle auto-advance on/off instead of requiring key hold
- **Wrap-around mode**: Toroidal topology option (edges connect)
- **Pattern library**: Built-in collection of interesting patterns with keyboard shortcuts
- **Color schemes**: Customizable colors for cells
- **Zoom controls**: Scale view in/out for different board sizes
- **Pan controls**: Move viewport for boards larger than window
- **Mouse drawing mode**: Click and drag to paint multiple cells

### Advanced Features

- **Statistics panel**: Track max population, generation of last change, stable detection
- **Undo/Redo**: History for cell editing (before simulation starts)
- **Grid toggle**: Option to show/hide cell grid lines
- **Step backward**: Record history to allow reverse stepping
- **Speed multiplier**: Fast-forward through multiple generations per frame
- **Cell aging**: Color cells by age (how many generations they've been alive)
- **Drawing modes**: Line tool, rectangle fill, brush sizes

### Technical Improvements

- **Optimize rendering**: Only redraw changed cells
- **Optimize tick**: Use hashset for sparse boards, or quad-tree for large empty areas
- **Benchmark suite**: Performance tests for large boards
- **Error handling**: Better error messages for window creation failures
