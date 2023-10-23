use proconio::input;
// o == white
// x == black

const BOARD_HEIGHT: usize = 10;
const BOARD_WIDTH: usize = 10;
const WALL: char = '#';
const EMPTY: char = '-';
const WHITE: char = 'o';
const BLACK: char = 'x';
const INDEX: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

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
    let mut n_vec = Vec::new();
    let mut n_y = *y;
    let n_x = *x;
    while board[n_y][n_x] != WALL {
        n_y -= 1;
        if board[n_y][n_x] == EMPTY {
            break;
        }
        if board[n_y][n_x] != player {
            n_vec.push((n_y, n_x));
        } else if board[n_y][n_x] == player {
            for i in &n_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // NE
    let mut ne_vec = Vec::new();
    let mut ne_y = *y;
    let mut ne_x = *x;
    while board[ne_y][ne_x] != WALL {
        ne_y -= 1;
        ne_x += 1;
        if board[ne_y][ne_x] == EMPTY {
            break;
        }
        if board[ne_y][ne_x] != player {
            ne_vec.push((ne_y, ne_x));
        } else if board[ne_y][ne_x] == player {
            for i in &ne_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // E
    let mut e_vec = Vec::new();
    let e_y = *y;
    let mut e_x = *x;
    while board[e_y][e_x] != WALL {
        e_x += 1;
        if board[e_y][e_x] == EMPTY {
            break;
        }
        if board[e_y][e_x] != player {
            e_vec.push((e_y, e_x));
        } else if board[e_y][e_x] == player {
            for i in &e_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // SE
    let mut se_vec = Vec::new();
    let mut se_y = *y;
    let mut se_x = *x;
    while board[se_y][se_x] != WALL {
        se_y += 1;
        se_x += 1;
        if board[se_y][se_x] == EMPTY {
            break;
        }
        if board[se_y][se_x] != player {
            se_vec.push((se_y, se_x));
        } else if board[se_y][se_x] == player {
            for i in &se_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // S
    let mut s_vec = Vec::new();
    let mut s_y = *y;
    let s_x = *x;
    while board[s_y][s_x] != WALL {
        s_y += 1;
        if board[s_y][s_x] == EMPTY {
            break;
        }
        if board[s_y][s_x] != player {
            s_vec.push((s_y, s_x));
        } else if board[s_y][s_x] == player {
            for i in &s_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // SW
    let mut sw_vec = Vec::new();
    let mut sw_y = *y;
    let mut sw_x = *x;
    while board[sw_y][sw_x] != WALL {
        sw_y += 1;
        sw_x -= 1;
        if board[sw_y][sw_x] == EMPTY {
            break;
        }
        if board[sw_y][sw_x] != player {
            sw_vec.push((sw_y, sw_x));
        } else if board[sw_y][sw_x] == player {
            for i in &sw_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // W
    let mut w_vec = Vec::new();
    let w_y = *y;
    let mut w_x = *x;
    while board[w_y][w_x] != WALL {
        w_x -= 1;
        if board[w_y][w_x] == EMPTY {
            break;
        }
        if board[w_y][w_x] != player {
            w_vec.push((w_y, w_x));
        } else if board[w_y][w_x] == player {
            for i in &w_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }

    // NW
    let mut nw_vec = Vec::new();
    let mut nw_y = *y;
    let mut nw_x = *x;
    while board[nw_y][nw_x] != WALL {
        nw_y -= 1;
        nw_x -= 1;
        if board[nw_y][nw_x] == EMPTY {
            break;
        }
        if board[nw_y][nw_x] != player {
            nw_vec.push((nw_y, nw_x));
        } else if board[nw_y][nw_x] == player {
            for i in &nw_vec {
                if player == BLACK {
                    board[i.0][i.1] = BLACK;
                } else if player == WHITE {
                    board[i.0][i.1] = WHITE;
                }
            }
        }
    }
}

fn check_can_put(
    board: &[[char; BOARD_WIDTH]; BOARD_HEIGHT],
    player: char,
    y: &usize,
    x: &usize,
) -> Result<(), String> {
    if board[*y][*x] != EMPTY {
        return Err("そこにはおけないよ".to_string());
    }

    // N
    let mut check_n = false;
    let mut n_y = *y;
    let n_x = *x;
    while board[n_y][n_x] != WALL {
        n_y -= 1;
        if board[n_y][n_x] == player {
            if check_n {
                return Ok(());
            }
        } else if board[n_y][n_x] != '-' {
            check_n = true;
            continue;
        } else {
            break;
        }
    }

    // NE
    let mut check_ne = false;
    let mut ne_y = *y;
    let mut ne_x = *x;
    while board[ne_y][ne_x] != WALL {
        ne_y -= 1;
        ne_x += 1;
        if board[ne_y][ne_x] == player {
            if check_ne {
                return Ok(());
            }
        } else if board[ne_y][ne_x] != EMPTY {
            check_ne = true;
            continue;
        } else {
            break;
        }
    }

    // E
    let mut check_e = false;
    let e_y = *y;
    let mut e_x = *x;
    while board[e_y][e_x] != WALL {
        e_x += 1;
        if board[e_y][e_x] == player {
            if check_e {
                return Ok(());
            }
        } else if board[e_y][e_x] != EMPTY {
            check_e = true;
            continue;
        } else {
            break;
        }
    }

    // SE
    let mut check_se = false;
    let mut se_y = *y;
    let mut se_x = *x;
    while board[se_y][se_x] != WALL {
        se_y += 1;
        se_x += 1;
        if board[se_y][se_x] == player {
            if check_se {
                return Ok(());
            }
        } else if board[se_y][se_x] != EMPTY {
            check_se = true;
            continue;
        } else {
            break;
        }
    }

    // S
    let mut check_s = false;
    let mut s_y = *y;
    let s_x = *x;
    while board[s_y][s_x] != WALL {
        s_y += 1;
        if board[s_y][s_x] == player {
            if check_s {
                return Ok(());
            }
        } else if board[s_y][s_x] != EMPTY {
            check_s = true;
            continue;
        } else {
            break;
        }
    }

    // SW
    let mut check_sw = false;
    let mut sw_y = *y;
    let mut sw_x = *x;
    while board[sw_y][sw_x] != WALL {
        sw_y += 1;
        sw_x -= 1;
        if board[sw_y][sw_x] == player {
            if check_sw {
                return Ok(());
            }
        } else if board[sw_y][sw_x] != EMPTY {
            check_sw = true;
            continue;
        } else {
            break;
        }
    }

    // W
    let mut check_w = false;
    let w_y = *y;
    let mut w_x = *x;
    while board[w_y][w_x] != WALL {
        w_x -= 1;
        if board[w_y][w_x] == player {
            if check_w {
                return Ok(());
            }
        } else if board[w_y][w_x] != EMPTY {
            check_w = true;
            continue;
        } else {
            break;
        }
    }

    // NW
    let mut check_nw = false;
    let mut nw_y = *y;
    let mut nw_x = *x;
    while board[nw_y][nw_x] != WALL {
        nw_y -= 1;
        nw_x -= 1;
        if board[nw_y][nw_x] == player {
            if check_nw {
                return Ok(());
            }
        } else if board[nw_y][nw_x] != EMPTY {
            check_nw = true;
            continue;
        } else {
            break;
        }
    }

    return Err("そこにはおけないよ".to_string());
}

fn main() {
    let mut turn = 1;

    let mut board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = [[EMPTY; BOARD_WIDTH]; BOARD_HEIGHT];
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
