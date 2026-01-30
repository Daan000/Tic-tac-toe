use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug,PartialEq)]
pub enum Tile {
    Empty,
    Circle,
    Cross
}
pub struct Board {
    tiles: [Tile; 9]
}
impl Board {
    pub fn set(&mut self, pos:usize, t:Tile){
        self.tiles[pos] = t
    }
    pub fn check_win(&self,t:Tile)->bool{
        let board = self.tiles;


        //vertical
        for i in 0..3{
            if board[i] == t && board[i+3] == t && board[i+6] == t{
                return true
            }
        }
        //horizontal
        for i in 0..3{
            if board[i+ 3*i] == t && board[1+ 3*i] == t && board[2 + 3*i] == t{
                return true
            }
        }
        //diagonal


        false
    }
    pub fn new() -> Board {
        Board { tiles: [Tile::Empty; 9] }
    }
    pub fn show(&self){
        println!("{:?}",self.tiles)

    }
}