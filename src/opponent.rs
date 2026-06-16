use crate::board::Board;
use crate::board::Tile;
use rand::seq::IteratorRandom;

pub trait Opponent {
    fn get_next_move(&self, board: &Board, team: Tile) -> usize;
    fn get_description(&self) -> String;
}
pub struct SimpleOpponent;
impl Opponent for SimpleOpponent {
    fn get_next_move(&self, board: &Board, _team: Tile) -> usize {
        let b = board.get_board();

        b.into_iter()
            .position(|x| matches!(x, Tile::Empty))
            .unwrap()
    }
    fn get_description(&self) -> String {
        String::from("a simple opponent that picks the first empty tile")
    }
}
pub struct RandomOpponent;
impl Opponent for RandomOpponent {
    fn get_next_move(&self, board: &Board, _team: Tile) -> usize {
        let b = board.get_board();
        b.into_iter()
            .enumerate()
            .filter(|(_, x)| matches!(x, Tile::Empty))
            .map(|(i, _)| i)
            .choose(&mut rand::rng())
            .unwrap()
    }
    fn get_description(&self) -> String {
        String::from("a random opponent that picks a random empty tile")
    }
}
