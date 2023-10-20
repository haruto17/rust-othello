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
    print!("  ");
    for i in 1..=8 {
        print!("{}", i);
    }

    println!();

    for i in 1..=8 {
        print!("{} ", i);
        for j in 1..=8 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn update_board(
    board: &mut [[char; BOARD_WIDTH]; BOARD_HEIGHT],
    player: usize,
    x: &usize,
    y: &usize,
) {
    if player == 1 {
        board[*x][*y] = BLACK;

        // N
        let n_x = *x;
        let mut n_y = *y;
        while board[n_x][n_y] != WALL {
            if board[n_x][n_y] == WHITE {
                board[n_x][n_y] = BLACK;
            } else {
                break;
            }
            n_y -= 1;
        }

        // NE
        let mut ne_x = *x;
        let mut ne_y = *y;
        while board[ne_x][ne_y] != WALL {
            if board[ne_x][ne_y] == WHITE {
                board[ne_x][ne_y] = BLACK;
            } else {
                break;
            }
            ne_x += 1;
            ne_y -= 1;
        }

        // E
        let mut e_x = *x;
        let e_y = *y;
        while board[e_x][e_y] != WALL {
            if board[e_x][e_y] == WHITE {
                board[e_x][e_y] = BLACK;
            } else {
                break;
            }
            e_x += 1;
        }

        // SE
        let mut se_x = *x;
        let mut se_y = *y;
        while board[se_x][se_y] != WALL {
            if board[se_x][se_y] == WHITE {
                board[se_x][se_y] = BLACK;
            } else {
                break;
            }
            se_x += 1;
            se_y += 1;
        }

        // S
        let s_x = *x;
        let mut s_y = *y;
        while board[s_x][s_y] != WALL {
            if board[s_x][s_y] == WHITE {
                board[s_x][s_y] = BLACK;
            } else {
                break;
            }
            s_y += 1;
        }

        // SW
        let mut sw_x = *x;
        let mut sw_y = *y;
        while board[sw_x][sw_y] != WALL {
            if board[sw_x][sw_y] == WHITE {
                board[sw_x][sw_y] = BLACK;
            } else {
                break;
            }
            sw_x -= 1;
            sw_y += 1;
        }

        // W
        let mut w_x = *x;
        let w_y = *y;
        while board[w_x][w_y] != WALL {
            if board[w_x][w_y] == WHITE {
                board[w_x][w_y] = BLACK;
            } else {
                break;
            }
            w_x -= 1;
        }

        // NW
        let mut nw_x = *x;
        let mut nw_y = *y;
        while board[nw_x][nw_y] != WALL {
            if board[nw_x][nw_y] == WHITE {
                board[n_x][n_y] = BLACK;
            } else {
                break;
            }
            nw_x -= 1;
            nw_y -= 1;
        }
    } else {
        board[*x][*y] = WHITE;

        // N
        let n_x = *x;
        let mut n_y = *y;
        while board[n_x][n_y] != WALL {
            if board[n_x][n_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            n_y -= 1;
        }

        // NE
        let mut ne_x = *x;
        let mut ne_y = *y;
        while board[ne_x][ne_y] != WALL {
            if board[n_x][n_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            ne_x += 1;
            ne_y -= 1;
        }

        // E
        let mut e_x = *x;
        let e_y = *y;
        while board[e_x][e_y] != WALL {
            if board[e_x][e_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            e_x += 1;
        }

        // SE
        let mut se_x = *x;
        let mut se_y = *y;
        while board[se_x][se_y] != WALL {
            if board[se_x][se_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            se_x += 1;
            se_y += 1;
        }

        // S
        let s_x = *x;
        let mut s_y = *y;
        while board[s_x][s_y] != WALL {
            if board[s_x][s_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            s_y += 1;
        }

        // SW
        let mut sw_x = *x;
        let mut sw_y = *y;
        while board[sw_x][sw_y] != WALL {
            if board[sw_x][sw_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            sw_x -= 1;
            sw_y += 1;
        }

        // W
        let mut w_x = *x;
        let w_y = *y;
        while board[w_x][w_y] != WALL {
            if board[w_x][w_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            w_x -= 1;
        }

        // NW
        let mut nw_x = *x;
        let mut nw_y = *y;
        while board[nw_x][nw_y] != WALL {
            if board[nw_x][nw_y] == BLACK {
                board[n_x][n_y] = WHITE;
            } else {
                break;
            }
            nw_x -= 1;
            nw_y -= 1;
        }
    }
}

fn check_can_put(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    player: char,
    y: &usize,
    x: &usize,
) -> Result<(), ()> {
    // N
    let mut check_n: bool = false;
    let mut n_y = *y;
    let n_x = *x;
    while board[n_y][n_x] != WALL {
        if board[n_y][n_x] == player {
            if check_n {
                return Ok(());
            }
        }
        if board[n_y][n_x] != player {
            check_n = true;
        }
        n_y -= 1;
    }

    // NE
    let mut check_ne: bool = false;
    let mut ne_y = *y;
    let mut ne_x = *x;
    while board[ne_y][ne_x] != WALL {
        if board[ne_y][ne_x] == player {
            if check_ne == true {
                return Ok(());
            }
        }
        if board[ne_y][ne_x] != player {
            check_ne = true;
        }
        ne_y -= 1;
        ne_x += 1;
    }

    // E
    let mut check_e: bool = false;
    let e_y = *y;
    let mut e_x = *x;
    while board[e_y][e_x] != WALL {
        if board[e_y][e_x] == player {
            if check_e {
                return Ok(());
            }
        }
        if board[e_y][e_x] != player {
            check_e = true;
        }
        e_x += 1;
    }

    // SE
    let mut check_se: bool = false;
    let mut se_y = *y;
    let mut se_x = *x;
    while board[se_y][se_x] != WALL {
        if board[se_y][se_x] == player {
            if check_se {
                return Ok(());
            }
        }
        if board[se_y][se_x] != player {
            check_se = true;
        }
        se_y += 1;
        se_x += 1;
    }

    // S
    let mut check_s: bool = false;
    let mut s_y = *y;
    let s_x = *x;
    while board[s_y][s_x] != WALL {
        if board[s_y][s_x] == player {
            if check_s {
                return Ok(());
            }
        }
        if board[s_y][s_x] != player {
            check_s = true;
        }
        s_y += 1;
    }

    // SW
    let mut check_sw: bool = false;
    let mut sw_y = *y;
    let mut sw_x = *x;
    while board[sw_y][sw_x] != WALL {
        if board[sw_y][sw_x] == player {
            if check_sw {
                return Ok(());
            }
        }
        if board[sw_y][sw_x] != player {
            check_sw = true;
        }
        sw_y += 1;
        sw_x -= 1;
    }

    // W
    let mut check_w: bool = false;
    let w_y = *y;
    let mut w_x = *x;
    while board[w_y][w_x] != WALL {
        if board[w_y][w_x] == player {
            if check_w {
                return Ok(());
            }
        }
        if board[w_y][w_x] != player {
            check_w = true;
        }
        w_x -= 1;
    }

    // NW
    let mut check_nw: bool = false;
    let mut nw_y = *y;
    let mut nw_x = *x;
    while board[nw_y][nw_x] != WALL {
        if board[nw_y][nw_x] == player {
            if check_nw {
                return Ok(());
            }
        }
        if board[nw_y][nw_x] != player {
            check_nw = true;
        }
        nw_y -= 1;
        nw_x -= 1;
    }
    return Err(());
}

fn main() {
    let mut turn = 1;

    let mut board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = [['-'; BOARD_WIDTH]; BOARD_HEIGHT];
    init_board(&mut board);

    loop {
        if turn % 2 != 0 {
            println!("Turn {}: {BLACK}", turn);
            draw_board(&board);
            println!("y x");
            input! {
                y: usize,
                x: usize,
            }
            match check_can_put(&board, BLACK, &y, &x) {
                Ok(()) => {
                    update_board(&mut board, 1, &y, &x);
                    turn += 1;
                }
                Err(()) => continue,
            }
        } else {
            println!("Turn {}: {WHITE}", turn);
            draw_board(&board);
            println!("y x");
            input! {
                y: usize,
                x: usize,
            }
            match check_can_put(&board, WHITE, &y, &x) {
                Ok(()) => {
                    update_board(&mut board, 2, &y, &x);
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
