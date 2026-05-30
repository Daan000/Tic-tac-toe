use crate::board::Board;
use crate::board::Tile;

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
