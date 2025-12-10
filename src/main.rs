// Hide console window on windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Module declarations for board and player components
mod board;

// Import my custom types
use board::Board;

// Import Piston window and graphics libraries
use piston_window;
use piston_window::graphics::{Context, Graphics, clear, rectangle};
use piston_window::*;

// Import timing utilities for auto-advance
use std::time::{Duration, Instant};

// Import HashSet for tracking pressed keys
use std::collections::HashSet;

fn main() {
    // Configuration constants for the game window
    const CELL_SIZE: f64 = 20.0; // Size of each cell in pixels
    const BOARD_WIDTH: i8 = 50; // Number of cells horizontally
    const BOARD_HEIGHT: i8 = 50; // Number of cells vertically
    const FPS: u8 = 60; // This is the fps of the game
    const AUTO_TICK_INTERVAL_MS: u64 = 50; // Milliseconds between auto ticks when F is held

    // Calculate window dimensions based on board size and cell size
    let window_width: u32 = (BOARD_WIDTH as f64 * CELL_SIZE) as u32;
    let window_height: u32 = (BOARD_HEIGHT as f64 * CELL_SIZE) as u32;

    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life", [window_width, window_height])
            .exit_on_esc(true) // Allow ESC key to close the window
            .resizable(false) // Prevent window resizing (may not work with Glutin)
            .samples(0) // Disable multisampling to avoid alpha mode issues
            .vsync(true) // Enable vsync for smoother rendering
            .build()
            .unwrap();

    // Initialize the game state
    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
    let mut generation: u128 = 0;

    let mut board_x: i8 = 0;
    let mut board_y: i8 = 0;

    // Track which keys are currently pressed
    let mut pressed_keys: HashSet<Key> = HashSet::new();

    // Track last auto-tick time for continuous advancement
    let mut last_auto_tick: Instant = Instant::now();

    let mut events: Events = Events::new(EventSettings::new().max_fps(FPS as u64));

    // Main game loop - processes events and renders frames
    while let Some(event) = events.next(&mut window) {
        // Track key press events
        if let Some(Button::Keyboard(key)) = event.press_args() {
            pressed_keys.insert(key);

            // Handle one time press actions
            match key {
                Key::Right => {
                    board.tick();
                    generation += 1;
                }
                Key::R => {
                    board.clear_board();
                    generation = 0;
                }
                _ => {}
            }
        }

        // Keeps track of mouse and board positions
        if let Some(cursor_pos) = event.mouse_cursor_args() {
            let mouse_x: f64 = cursor_pos[0];
            let mouse_y: f64 = cursor_pos[1];

            board_x = (mouse_x / CELL_SIZE) as i8;
            board_y = (mouse_y / CELL_SIZE) as i8;
        }

        // If you click it toggles the cell at the mouse position
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            board.player_toggle_cell(board_x, board_y);
        }

        // Track key release events
        if let Some(Button::Keyboard(key)) = event.release_args() {
            pressed_keys.remove(&key);
        }

        // Handle continuous key press actions
        if pressed_keys.contains(&Key::F) {
            // Check if enough time has passed since the last auto tick
            if last_auto_tick.elapsed() >= Duration::from_millis(AUTO_TICK_INTERVAL_MS) {
                board.tick();
                generation += 1;
                last_auto_tick = Instant::now();
            }
        }

        // Render the game on each frame
        window.draw_2d(&event, |context, graphics, _device| {
            // Clear the screen with a dark background color
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            // Draw the UI text at the top of the window
            draw_ui(&board, generation, &context);

            // Draw each cell of the board
            draw_board(&board, CELL_SIZE, &context, graphics);
        });
    }
}

// Draws the user interface text showing controls (soon to be)
fn draw_ui(_board: &Board, _generation: u128, _context: &Context) {
    // I will add this later
}

// Draws the game board with all cells and the player cursor
fn draw_board<G: Graphics>(board: &Board, cell_size: f64, context: &Context, graphics: &mut G) {
    // Define colors for different cell states
    let alive_color: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // RBGA color skema (black)
    let dead_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // RBGA color skema (white)

    // Offset for the board (no offset for now)
    let y_offset: f64 = 0.0;
    let x_offset: f64 = 0.0;

    // Iterate through each cell in the board
    for y in 0..board.size_y {
        for x in 0..board.size_x {
            // Calculate the pixel position of this cell
            let x_pos = x as f64 * cell_size + x_offset;
            let y_pos = y as f64 * cell_size + y_offset;

            // Create a rectangle for this cell with 1px gap between cells for visual stuff
            let cell_rect: [f64; 4] = [x_pos, y_pos, cell_size - 1.0, cell_size - 1.0];

            // Determine the cell's color based on its state (alive or dead)
            let color: [f32; 4] = if board.board[y as usize][x as usize] == 1 {
                alive_color
            } else {
                dead_color
            };

            // Draw the cell rectangle
            rectangle(color, cell_rect, context.transform, graphics);
        }
    }
}
