use tic_tac_toe::Board;
use tic_tac_toe::Tile;

fn main() {
    let mut b = Board::new();
    b.show();
    b.set(6,Tile::Circle);
    b.set(7,Tile::Circle);
    b.set(8,Tile::Circle);

    b.show();
    println!("{}",b.check_win(Tile::Circle));
}
