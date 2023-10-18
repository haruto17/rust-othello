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

fn check_can_put(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    player: char,
    x: &usize,
    y: &usize,
) -> Result<(), ()> {
    // N
    let mut check_n: bool = false;
    let n_x = *x;
    let mut n_y = *y;
    while board[n_x][n_y] != '#' {
        if board[n_x][n_y] == player {
            if check_n {
                return Ok(());
            }
        }

        if board[n_x][n_y] != player {
            check_n = true;
        }

        n_y -= 1;
    }

    // NE
    let mut check_ne: bool = false;
    let mut ne_x = *x;
    let mut ne_y = *y;
    while board[ne_x][ne_y] != '#' {
        if board[ne_x][ne_y] == player {
            if check_ne {
                return Ok(());
            }
        }

        if board[ne_x][ne_y] != player {
            check_ne = true;
        }

        ne_x += 1;
        ne_y -= 1;
    }

    // E
    let mut check_e: bool = false;
    let mut e_x = *x;
    let e_y = *y;
    while board[e_x][e_y] != '#' {
        if board[e_x][e_y] == player {
            if check_e {
                return Ok(());
            }
        }

        if board[e_x][e_y] != player {
            check_e = true;
        }

        e_x += 1;
    }

    // SE
    let mut check_se: bool = false;
    let mut se_x = *x;
    let mut se_y = *y;
    while board[se_x][se_y] != '#' {
        if board[se_x][se_y] == player {
            if check_se {
                return Ok(());
            }
        }

        if board[se_x][se_y] != player {
            check_se = true;
        }

        se_x += 1;
        se_y += 1;
    }

    // S
    let mut check_s: bool = false;
    let s_x = *x;
    let mut s_y = *y;
    while board[s_x][s_y] != '#' {
        if board[s_x][s_y] == player {
            if check_s {
                return Ok(());
            }
        }

        if board[s_x][s_y] != player {
            check_s = true;
        }

        s_y += 1;
    }

    // SW
    let mut check_sw: bool = false;
    let mut sw_x = *x;
    let mut sw_y = *y;
    while board[sw_x][sw_y] != '#' {
        if board[sw_x][sw_y] == player {
            if check_sw {
                return Ok(());
            }
        }

        if board[sw_x][sw_y] != player {
            check_sw = true;
        }

        sw_x -= 1;
        sw_y += 1;
    }

    // W
    let mut check_w: bool = false;
    let mut w_x = *x;
    let w_y = *y;
    while board[w_x][w_y] != '#' {
        if board[w_x][w_y] == player {
            if check_w {
                return Ok(());
            }
        }

        if board[w_x][w_y] != player {
            check_w = true;
        }

        w_x -= 1;
    }

    // NW
    let mut check_nw: bool = false;
    let mut nw_x = *x;
    let mut nw_y = *y;
    while board[nw_x][nw_y] != '#' {
        if board[nw_x][nw_y] == player {
            if check_nw {
                return Ok(());
            }
        }

        if board[nw_x][nw_y] != player {
            check_nw = true;
        }

        nw_x -= 1;
        nw_y -= 1;
    }
    Err(())
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
            match check_can_put(&board, BLACK, &x, &y) {
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
            match check_can_put(&board, WHITE, &x, &y) {
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
