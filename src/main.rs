use proconio::input;
// o == white
// x == black

const BOARD_HEIGHT: usize = 10;
const BOARD_WIDTH: usize = 10;

fn init_board(board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT]) {
    board[4][4] = 'o';
    board[4][5] = 'x';
    board[5][4] = 'x';
    board[5][5] = 'o';
    for i in 0..10 {
        board[0][i] = '#';
        board[i][0] = '#';
        board[9][i] = '#';
        board[i][9] = '#';
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
        board[x][y] = 'x';
    } else {
        board[x][y] = 'o';
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
            println!("Turn {}: x", turn);
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
            println!("Turn {}: o", turn);
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
