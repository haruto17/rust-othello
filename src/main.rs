// o == white
// x == black

fn init_board(board: &mut [[char; 8]; 8]) {
    board[3][3] = 'o';
    board[3][4] = 'x';
    board[4][3] = 'x';
    board[4][4] = 'o';
}

fn draw_board(board: &[[char; 8]; 8]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut turn = 1;

    let mut board: [[char; 8]; 8] = [['-'; 8]; 8];
    init_board(&mut board);

    loop {
        if turn % 2 != 0 {
            println!("Turn {}: o", turn);
            draw_board(&board);
            turn += 1;
        } else {
            println!("Turn {}: x", turn);
            draw_board(&board);
            turn += 1;
        }

        println!();

        if turn == 10 {
            break;
        }
    }
}
