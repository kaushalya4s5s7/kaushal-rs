use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];


fn init ()-> Board{
  return [[' ';BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: Board) {
    for row in board{
        for cell in row{
            print!("{}", cell);
        }
        println!();
    }
}

fn get_player_move(current_player: char , board: Board)->(usize,usize){
   println!("Player {}'s turn. Enter your move (row and column):", current_player);
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("unable to read the input");
   println!("You entered: {}", input);

   let coordinates: Vec<usize> = input
       .split_whitespace()
       .flat_map(str::parse::<usize>)
       .collect();

    if coordinates.len()==2{
        let row = coordinates[0];
        let column = coordinates[1];
        if row < BOARD_SIZE && column < BOARD_SIZE && board[row][column] == ' ' {
            return (row, column);
        } else {
            println!("Invalid move. Try again.");
            return get_player_move(current_player, board);
        }
    } else {
        println!("Invalid input. Please enter two numbers separated by space.");
        return get_player_move(current_player, board);
    }
}

fn check_winner( current_player: char, board: Board)-> bool{
  for row in 0..BOARD_SIZE{
    if board[row][0]== current_player && board[row][1] == current_player && board[row][2] == current_player {
        return true;
    }
  }
    for col in 0..BOARD_SIZE{
        if board[0][col]== current_player && board[1][col] == current_player && board[2][col] == current_player {
            return true;
        }
    }
    if board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player || board[0][2] == current_player && board[1][1] == current_player && board[2][0] == current_player {
        return true;
    }
 return false;
} 

fn check_draw(board: Board)-> bool{
    for row in board{
        for cell in row{
            if cell==' '{
                return false;
        }
    }
}
return true}

fn play_game(){
    let mut board = init();
    let mut current_player = PLAYER_X;
    
    loop{
        print!("current Board:");
        print_board(board.clone());
        if check_winner (current_player, board.clone()){
        println! ("Player {} is the winner", current_player); break;
        }
        if check_draw(board.clone()){
            println!("the game is draw");
        }
        let (row,column) = get_player_move(current_player,board.clone());
        board[row][column] = current_player;
         current_player = if current_player == PLAYER_X{
        PLAYER_O}
        else{
            PLAYER_X
        }
    }
    }

   




fn main() {
   println!("Welcome to Tic Tac Toe!");
   play_game();

}
