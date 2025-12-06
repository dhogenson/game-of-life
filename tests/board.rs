use game_of_life::Board;

#[test]
fn test_new_board_is_empty() {
    let board = Board::new(10, 10);
    assert_eq!(board.population, 0);
}

#[test]
fn test_blinker_pattern() {
    let mut board = Board::new(5, 5);

    // Set up horizontal blinker at row 2
    board.board[2][1] = 1;
    board.board[2][2] = 1;
    board.board[2][3] = 1;
    board.population = 3;

    // Advance one generation
    board.tick();

    // Should become vertical blinker at column 2
    assert_eq!(board.board[1][2], 1);
    assert_eq!(board.board[2][2], 1);
    assert_eq!(board.board[3][2], 1);
    assert_eq!(board.population, 3);
}

#[test]
fn test_neighbour_count() {
    let mut board = Board::new(3, 3);

    // Make all cells alive
    for y in 0..3 {
        for x in 0..3 {
            board.board[y as usize][x as usize] = 1
        }
    }

    assert_eq!(board.get_neighbour_count(1, 1), 8);

    board.board[0][0] = 0; // Make one cell dead

    assert_eq!(board.get_neighbour_count(1, 1), 7);
}
