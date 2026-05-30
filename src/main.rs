use std::io;
use std::io::Write;

use tic_tac_toe::{Game, GameStatus, Opponent, SimpleOpponent, Tile};

fn main() {
    //gemini bullshit loop
    let mut game = Game::new(Tile::Cross);
    let opp = SimpleOpponent;

    println!("Welcome to Tic Tac Toe!");
    println!("You are playing against {}", opp.get_description());

    while matches!(game.status(), GameStatus::Ongoing) {
        game.show_board();

        print!(
            "Player {:?}, make your move (row column): ",
            game.current_turn()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Read of input failed");

        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() != 2 || coords[0] > 2 || coords[1] > 2 {
            println!("Faulty input. Enter 2 numbers (0 or 1 or 2) separated by a space.");
            continue;
        }

        game.make_move(coords[0] * 3 + coords[1]);

        if !matches!(game.status(), GameStatus::Ongoing) {
            break;
        }

        println!("zet van de ai");
        game.make_move(opp.get_next_move(game.get_board(), game.current_turn()));
    }
    game.show_board();
    match game.status() {
        GameStatus::Ongoing => {
            panic!("Game is still ongoing but finished the loop")
        }
        GameStatus::Won(winner) => println!("Player {:?} won!", winner),
        GameStatus::Draw => println!("Draw!"),
    }
}
