use std::io;

fn main() {

    // defining tic tac toe board
    let mut board = ["1", "2","3", "4","5","6","7","8","9"];
    // defining player
    let mut p = "X";
    
    // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // game loop
    loop {
        displ(board);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't get user input.");
    
        match input.trim() {
            "1" => {board[0] = p}
            "2" => {board[1] = p}
            "3" => {board[2] = p}
            "4" => {board[3] = p}
            "5" => {board[4] = p}
            "6" => {board[5] = p}
            "7" => {board[6] = p}
            "8" => {board[7] = p}
            "9" => {board[8] = p}
            _ => { p = switch_p(p);
                   println!("Wrong input.")
                 }            
        }

        if check_winner(board){
            println!("{} wins", p);
            break
        }

        p = switch_p(p);
        
        
        //println!("\x1b[93mError\x1b[0m");
    }
}

// display board
fn displ(board: [&str; 9] ){
    for i in 0..3 {
        let offset = i * 3;
        print!("{}", board[offset]);
        print!(" | ");
        print!("{}", board[offset + 1]);
        print!(" | ");
        println!("{}", board[offset + 2]);
    }
}

// Switch player
fn switch_p(turn: &str) -> &str{
    let x = match turn {
        "O" =>  "X",
        "X" =>  "O",
         _ => "-",
    };

    x
}

fn check_winner(board: [&str; 9]) -> bool{

    // horizontal checks
    if board[0] == board[1] && board[2] == board[0] { return true }
    if board[3] == board[4] && board[5] == board[4] { return true }
    if board[6] == board[7] && board[8] == board[7] { return true }

    // vertical wins
    if board[0] == board[3] && board[6] == board[3] { return true }
    if board[1] == board[4] && board[7] == board[4] { return true }
    if board[2] == board[5] && board[8] == board[5] { return true }

    //  diagonal wins
    if board[0] == board[4] && board[8] == board[4] { return true }
    if board[2] == board[4] && board[6] == board[4] { return true }

    return false
}