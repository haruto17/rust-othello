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
    player: char,
    y: &usize,
    x: &usize,
) {
    if player == BLACK {
        board[*y][*x] = BLACK;
    } else if player == WHITE {
        board[*y][*x] = WHITE;
    }

    // N
    let mut n_y = *y;
    let n_x = *x;
    n_y -= 1;
    while board[n_y][n_x] != WALL {
        if player == BLACK {
            if board[n_y][n_x] == WHITE {
                board[n_y][n_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[n_y][n_x] == BLACK {
                board[n_y][n_x] = WHITE;
            } else {
                break;
            }
        }
        n_y -= 1;
    }

    // NE
    let mut ne_y = *y;
    let mut ne_x = *x;
    ne_y -= 1;
    ne_x += 1;
    while board[ne_y][ne_x] != WALL {
        if player == BLACK {
            if board[ne_y][ne_x] == WHITE {
                board[ne_y][ne_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[ne_y][ne_x] == BLACK {
                board[ne_y][ne_x] = WHITE;
            } else {
                break;
            }
        }
        ne_y -= 1;
        ne_x += 1;
    }

    // E
    let e_y = *y;
    let mut e_x = *x;
    e_x += 1;
    while board[e_y][e_x] != WALL {
        if player == BLACK {
            if board[e_y][e_x] == WHITE {
                board[e_y][e_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[e_y][e_x] == BLACK {
                board[e_y][e_x] = WHITE;
            } else {
                break;
            }
        }
        e_x += 1;
    }

    // SE
    let mut se_y = *y;
    let mut se_x = *x;
    se_y += 1;
    se_x += 1;
    while board[se_y][se_x] != WALL {
        if player == BLACK {
            if board[se_y][se_x] == WHITE {
                board[se_y][se_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[se_y][se_x] == BLACK {
                board[se_y][se_x] = WHITE;
            } else {
                break;
            }
        }
        se_y += 1;
        se_x += 1;
    }

    // S
    let mut s_y = *y;
    let s_x = *x;
    s_y += 1;
    while board[s_y][s_x] != WALL {
        if player == BLACK {
            if board[s_y][s_x] == WHITE {
                board[s_y][s_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[s_y][s_x] == BLACK {
                board[s_y][s_x] = WHITE;
            } else {
                break;
            }
        }
        s_y += 1;
    }

    // SW
    let mut sw_y = *y;
    let mut sw_x = *x;
    sw_y += 1;
    sw_x -= 1;
    while board[sw_y][sw_x] != WALL {
        if player == BLACK {
            if board[sw_y][sw_x] == WHITE {
                board[sw_y][sw_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[sw_y][sw_x] == BLACK {
                board[sw_y][sw_x] = WHITE;
            } else {
                break;
            }
        }
        sw_y += 1;
        sw_x -= 1;
    }

    // W
    let w_y = *y;
    let mut w_x = *x;
    w_x -= 1;
    while board[w_y][w_x] != WALL {
        if player == BLACK {
            if board[w_y][w_x] == WHITE {
                board[w_y][w_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[w_y][w_x] == BLACK {
                board[w_y][w_x] = WHITE;
            } else {
                break;
            }
        }
        w_x -= 1;
    }

    // NW
    let mut nw_y = *y;
    let mut nw_x = *x;
    nw_y -= 1;
    nw_x -= 1;
    while board[nw_y][nw_x] != WALL {
        if player == BLACK {
            if board[nw_y][nw_x] == WHITE {
                board[nw_y][nw_x] = BLACK;
            } else {
                break;
            }
        } else if player == WHITE {
            if board[nw_y][nw_x] == BLACK {
                board[nw_y][nw_x] = WHITE;
            } else {
                break;
            }
        }
        nw_y -= 1;
        nw_x -= 1;
    }
}

fn check_can_put(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    player: char,
    y: &usize,
    x: &usize,
) -> Result<(), String> {
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
        if board[n_y][n_x] != player && board[n_y][n_x] != '-' {
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
        if board[ne_y][ne_x] != player && board[ne_y][ne_x] != '-' {
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
        if board[e_y][e_x] != player && board[e_y][e_x] != '-' {
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
        if board[se_y][se_x] != player && board[se_y][se_x] != '-' {
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
        if board[s_y][s_x] != player && board[s_y][s_x] != '-' {
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
        if board[sw_y][sw_x] != player && board[sw_y][sw_x] != '-' {
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
        if board[w_y][w_x] != player && board[w_y][w_x] != '-' {
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
        if board[nw_y][nw_x] != player && board[nw_y][nw_x] != '-' {
            check_nw = true;
        }
        nw_y -= 1;
        nw_x -= 1;
    }
    return Err("そこにはおけないよ".to_string());
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
                    update_board(&mut board, BLACK, &y, &x);
                    turn += 1;
                }
                Err(e) => {
                    println!("{}", e);
                    println!();
                    continue;
                }
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
                    update_board(&mut board, WHITE, &y, &x);
                    turn += 1;
                }
                Err(e) => {
                    println!("{}", e);
                    println!();
                    continue;
                }
            }
        }

        println!();

        if turn == 10 {
            break;
        }
    }
}
