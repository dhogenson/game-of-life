// Represents the game board for Conway's Game of Life
// The board is a 2D grid where each cell can be either alive (1) or dead (0).
// It tracks the current state of all cells and maintains a count of living cells.

#[derive(Clone)]
pub struct Board {
    pub size_x: i8,           // Width of the board (number of columns)
    pub size_y: i8,           // Height of the board (number of rows)
    pub board: Vec<Vec<i8>>,  // 2D vector storing cell states (0=dead, 1=alive)
    next_board: Vec<Vec<i8>>, // Pre allocated buffer for next generation
    pub population: u64,      // Total number of living cells
}

impl Board {
    // Creates a new board with the specified dimensions
    // All cells are initialized to dead (0) and population starts at 0

    pub fn new(_size_x: i8, _size_y: i8) -> Self {
        // This returns the value of the struct
        Self {
            size_x: _size_x,
            size_y: _size_y,
            board: Self::make_board(_size_x, _size_y),
            next_board: Self::make_board(_size_x, _size_y),
            population: 0,
        }
    }

    // Creates a 2D vector making an empty board with all dead cells
    // A Vec<Vec<i8>> where all cells are initialized to 0 (dead)
    pub fn make_board(size_x: i8, size_y: i8) -> Vec<Vec<i8>> {
        let mut board: Vec<Vec<i8>> = Vec::new();

        // Create each row
        for _y in 0..size_y {
            let mut row: Vec<i8> = Vec::new();

            // Fill the row with dead cells
            for _x in 0..size_x {
                row.push(0);
            }

            board.push(row);
        }

        board
    }

    // Toggles the state of the cell at the player's mouse position
    pub fn player_toggle_cell(&mut self, board_x: i8, board_y: i8) {
        let cell: &mut i8 = &mut self.board[board_y as usize][board_x as usize];

        if *cell == 0 {
            // Cell is dead, make it alive
            *cell = 1;
            self.population += 1;
        } else {
            // Cell is alive, make it dead
            *cell = 0;
            self.population -= 1;
        }
    }

    // Resets the entire board to an empty state
    pub fn clear_board(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                self.board[y as usize][x as usize] = 0;
            }
        }
        self.population = 0;
    }

    // Advances the simulation by one generation using Conway's Game of Life rules
    // Here are the rules:
    // - Any live cell with 2 or 3 live neighbors survives
    // - Any dead cell with exactly 3 live neighbors becomes alive
    // - All other live cells die (underpopulation or overpopulation)
    // - All other dead cells stay dead
    pub fn tick(&mut self) {
        let mut new_population: u64 = 0;

        // Process each cell
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                // Count how many living neighbors this cell has
                let neighbour_count: i8 = self.get_neighbour_count(x, y);
                let current_state: i8 = self.board[y as usize][x as usize];

                // Apply Conway's Game of Life rules
                self.next_board[y as usize][x as usize] = if current_state == 1 {
                    // Cell is currently alive
                    if neighbour_count == 2 || neighbour_count == 3 {
                        1 // Survives
                    } else {
                        0 // Dies from underpopulation or overpopulation
                    }
                } else {
                    // Cell is currently dead
                    if neighbour_count == 3 {
                        1 // Becomes alive (reproduction)
                    } else {
                        0 // Stays dead
                    }
                };

                // Count living cells for the new population
                if self.next_board[y as usize][x as usize] == 1 {
                    new_population += 1;
                }
            }
        }

        // Swap the boards instead of copying
        std::mem::swap(&mut self.board, &mut self.next_board);
        self.population = new_population;
    }

    // Counts the number of living neighbors around a given cell
    // Checks all 8 adjacent cells (including diagonals). Cells outside
    // the board boundaries are considered dead.
    pub fn get_neighbour_count(&self, x: i8, y: i8) -> i8 {
        let mut neighbour_count: i8 = 0;

        // Check all 8 surrounding cells
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Skip the center cell (the cell itself)
                if dx == 0 && dy == 0 {
                    continue;
                }

                // Calculate neighbor coordinates
                let nx = x + dx;
                let ny = y + dy;

                // Only count neighbors that are within board boundaries
                if nx >= 0 && nx < self.size_x && ny >= 0 && ny < self.size_y {
                    neighbour_count += self.board[ny as usize][nx as usize];
                }
            }
        }

        neighbour_count // Return the value
    }
}
