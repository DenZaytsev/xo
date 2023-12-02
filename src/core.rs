use std::fmt;

use crate::field::{GameField, Cell};

#[derive(Debug)]
enum Player {
    X,
    O,
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "Player::X"),
            Player::O => write!(f, "Player::O"),
        }
    }
}


pub fn main_loop(mut field: GameField){

    let mut current_player = Player::X;
    let game_field = &mut field;

    loop {
        print_game_field(game_field, &current_player);

        let player_choise = get_user_input(&current_player);
        game_field.make_move(player_choise.0, player_choise.1);

        if game_field.check_win_condition() {
            println!("{current_player} WIN !!!");
            break
        }

        current_player = match current_player {
            Player::O => Player::X,
            Player::X => Player::O,
        };
    }

}


fn get_user_input(player: &Player) -> (u8, Cell){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read line! Try again.");

    let cell_num = input.trim().parse().expect("Couldn't read line! Try again.");
    let cell_val = match player {
            Player::O => Cell::O,
            Player::X => Cell::X,
    };


    return (cell_num, cell_val)

}


fn print_game_field(game_field: &GameField, player: &Player){
    println!("Current player: {:?}", player);
    println!("{}", game_field);
    println!("Choose your ceil: ");
}