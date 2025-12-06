# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a fully functional graphical implementation of Conway's Game of Life in Rust with an interactive editor. The game includes:

- Complete Game of Life simulation following Conway's standard rules
- Interactive cursor-based cell editor
- Real-time generation tracking and population display
- Graphical rendering using Piston game engine
- Colorized cell visualization (green = alive, dark gray = dead, red = cursor)
- Keyboard controls for cell manipulation and simulation advancement

**Status**: Core functionality complete. Game logic, rendering, and player controls are all implemented. UI text display is a placeholder for future enhancement.

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

- **WASD**: Move cursor around the board
- **Space**: Toggle cell state (alive/dead) at cursor position
- **Right Arrow (→)**: Advance simulation by one generation
- **R**: Reset/clear the board (all cells become dead, generation resets to 0)
- **ESC**: Quit the application

## Architecture

The codebase follows a modular structure with clear separation of concerns:

- **`src/main.rs`**: Entry point, window management, event loop, and rendering
- **`src/board.rs`**: Board struct with Game of Life logic
- **`src/player.rs`**: Player (cursor) struct for interactive editing

### Main Module (src/main.rs)

Entry point that:
- Initializes a 70x70 board and player cursor at (0, 0)
- Creates a Piston window (700x760 pixels: 700x700 board + 60px UI space)
- Sets up window with VSync enabled and ESC key to exit
- Runs the main event loop with Piston's event system
- Handles keyboard events and dispatches actions
- Renders the game board and UI on each frame using `draw_2d()`
- Tracks generation count (u128) and displays controls/stats

**Configuration Constants**:
- `CELL_SIZE`: 10.0 pixels per cell
- `BOARD_WIDTH`: 70 cells
- `BOARD_HEIGHT`: 70 cells

**Rendering Functions**:
- `draw_ui()`: Placeholder for UI text (controls, generation, population) - currently unimplemented (src/main.rs:83-89)
- `draw_board()`: Renders all cells as rectangles with color coding and cursor overlay

### Board Module (src/board.rs)

The `Board` struct represents the game grid:

- **Fields**:
  - `size_x`, `size_y`: Grid dimensions (i8)
  - `board: Vec<Vec<i8>>`: 2D grid storing cell states (0 = dead, 1 = alive)
  - `population: u64`: Current count of living cells

- **Methods**:
  - `new(size_x, size_y)`: Constructor that creates a board with specified dimensions
  - `make_board(size_x, size_y)`: Static method to initialize an empty grid
  - `player_toggle_cell(player)`: Toggles cell state at player position and updates population
  - `clear_board()`: Resets all cells to dead (0) and population to 0
  - `tick()`: Advances simulation by one generation following Conway's rules
  - `get_neighbour_count(x, y)`: Counts living neighbors for a given cell (used by tick)

### Player Module (src/player.rs)

The `Player` struct represents the cursor position:

- **Fields**:
  - `x`, `y`: Current cursor coordinates (i8)

- **Methods**:
  - `new()`: Constructor that initializes cursor at (0, 0)
  - `move_right(board)`, `move_left(board)`, `move_up(board)`, `move_down(board)`: Movement methods with boundary checking

## Dependencies

Listed in `Cargo.toml`:

- **piston_window** (0.130.0): 2D graphics and window management library
  - Used for: Window creation, event handling, 2D rendering, keyboard input
  - Provides: Graphics context, rectangle drawing, frame rendering, VSync

**Edition**: Rust 2024 (latest edition)

## Game of Life Rules

The implementation follows Conway's standard rules (implemented in `Board::tick()`):

- Any live cell with 2-3 live neighbors survives
- Any dead cell with exactly 3 live neighbors becomes alive
- All other live cells die (underpopulation or overpopulation)
- All other dead cells stay dead

## Rendering Details

### Visual Design

- **Window Size**: 700x760 pixels (700x700 game area + 60px top margin for UI)
- **Cell Size**: 10x10 pixels with 1px gap between cells
- **Background Color**: Dark gray [0.1, 0.1, 0.1, 1.0]
- **Living Cells**: Green [0.0, 1.0, 0.0, 1.0]
- **Dead Cells**: Dark gray [0.2, 0.2, 0.2, 1.0]
- **Cursor**: Semi-transparent red overlay [1.0, 0.0, 0.0, 0.5]

### Rendering Pipeline

1. Clear screen to dark gray background
2. Call `draw_ui()` for controls/stats (currently a placeholder)
3. Call `draw_board()` which:
   - Iterates through all cells (70x70 = 4900 cells)
   - Draws each cell as a rectangle with appropriate color
   - Overlays red rectangle at cursor position

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
- **Event handling**: Match on `Button::Keyboard(key)` for keyboard input
- **Drawing**: Call `window.draw_2d(&event, |context, graphics, _device| { ... })`

## Testing Strategy

Currently, no tests are implemented. When adding tests:

### Unit Tests

- Add `#[cfg(test)]` modules at the bottom of each source file
- Test `Board::get_neighbour_count()` with various cell configurations
- Test `Board::tick()` with known Game of Life patterns (blinkers, gliders, still lifes)
- Test `Board::clear_board()` to ensure it resets state correctly
- Test `Player` movement boundary conditions
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
        let player = Player::new();
        board.player_toggle_cell(player);
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
5. **Run tests**: Run `cargo test` (when tests exist)

### Making Changes

- **Small, focused commits**: Each commit should do one thing
- **Descriptive commit messages**: Explain what and why, not just what
- **Maintain existing patterns**: Follow the established code style
- **Update this file**: If you change architecture or add features, update CLAUDE.md

### Performance Considerations

- The game uses `clone()` liberally for simplicity; consider refactoring to use references for very large boards
- Piston handles rendering efficiently with VSync enabled
- The 70x70 grid (4900 cells) renders smoothly on modern hardware
- Release builds (`--release`) are recommended for best performance
- Board logic (tick) is fast; rendering is the primary bottleneck for very large boards

## Future Enhancements

Potential features to add:

### High Priority
- **Implement UI text display**: Complete the `draw_ui()` function to show controls, generation count, and population (src/main.rs:83-89)
- **Text rendering**: Integrate a text rendering solution (e.g., `piston_window::Glyphs`, `glyph_brush`)

### Feature Enhancements
- **Preset patterns**: Load famous Game of Life patterns (glider guns, spaceships, pulsars, etc.)
- **Save/Load**: Serialize board state to files (JSON, binary, or RLE format)
- **Configurable board size**: Command-line arguments for dimensions
- **Auto-advance mode**: Automatic simulation with adjustable tick rate (spacebar to pause/resume)
- **Wrap-around mode**: Toroidal topology option (edges connect)
- **Pattern library**: Built-in collection of interesting patterns with keyboard shortcuts
- **Color schemes**: Customizable colors for cells and cursor
- **Zoom controls**: Scale view in/out for different board sizes
- **Pan controls**: Move viewport for boards larger than window

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
