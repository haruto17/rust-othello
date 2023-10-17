use proconio::input;
// o == white
// x == black

const BOARD_HEIGHT: usize = 10;
const BOARD_WIDTH: usize = 10;
const WALL: char = '#';
const WHITE: char = 'o';
const BLACK: char = 'x';

fn init_board(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    board[4][4] = WHITE;
    board[4][5] = BLACK;
    board[5][4] = BLACK;
    board[5][5] = WHITE;
    for i in 0..10 {
        board[0][i] = WALL;
        board[i][0] = WALL;
        board[9][i] = WALL;
        board[i][9] = WALL;
    }
}

fn draw_board(board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 1..=8 {
        for j in 1..=8 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn put_stone(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT], player: usize, x: usize, y: usize) {
    if player == 1 {
        board[x][y] = BLACK;
    } else {
        board[x][y] = WHITE;
    }
}

fn check_can_put(x: usize, y: usize) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let mut turn = 1;

    let mut board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = [['-'; BOARD_WIDTH]; BOARD_HEIGHT];
    init_board(&mut board);

    loop {
        if turn % 2 != 0 {
            println!("Turn {}: {BLACK}", turn);
            draw_board(&board);
            input! {
                x: usize,
                y: usize,
            }
            match check_can_put(x, y) {
                Ok(()) => {
                    put_stone(&mut board, 1, x, y);
                    turn += 1;
                }
                Err(()) => continue,
            }
        } else {
            println!("Turn {}: {WHITE}", turn);
            draw_board(&board);
            input! {
                x: usize,
                y: usize,
            }
            match check_can_put(x, y) {
                Ok(()) => {
                    put_stone(&mut board, 2, x, y);
                    turn += 1;
                }
                Err(()) => continue,
            }
        }

        println!();

        if turn == 10 {
            break;
        }
    }
}
