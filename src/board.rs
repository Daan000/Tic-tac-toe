use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Circle,
    Cross,
}
pub struct Board {
    tiles: [Tile; 9],
}
impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [Tile::Empty; 9],
        }
    }

    pub fn set(&mut self, pos: usize, t: Tile) {
        if pos > 8 {
            panic!("pos larger than 8")
        }
        self.tiles[pos] = t;
    }
    pub fn check_win(&self, team: Tile) -> bool {
        let board = self.tiles;
        // vertical
        for i in 0..3 {
            if board[i] == team && board[i + 3] == team && board[i + 6] == team {
                return true;
            }
        }
        // horizontal
        for i in 0..3 {
            let row = i * 3;
            if board[row] == team && board[row + 1] == team && board[row + 2] == team {
                return true;
            }
        }
        // diagonal
        if board[0] == team && board[4] == team && board[8] == team {
            return true;
        }
        if board[2] == team && board[4] == team && board[6] == team {
            return true;
        }

        false
    }
    pub fn check_empty_tile(&self, pos: usize) -> bool {
        if pos > 8 {
            panic!("pos greater then 8")
        }
        self.tiles[pos] == Tile::Empty
    }
    pub fn check_full_board(&self) -> bool {
        self.tiles
            .iter()
            .all(|t| matches!(t, Tile::Circle) || matches!(t, Tile::Cross))
    }
    pub fn get_board(&self) -> [Tile; 9] {
        self.tiles
    }

    pub fn show(&self) {
        println!("{:?}", self.tiles)
    }
    pub fn print_grid(&self) {
        for (i, cel) in self.tiles.iter().enumerate() {
            print!("{} ", cel);

            if (i + 1) % 3 == 0 {
                println!();
            }
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
impl Tile {
    pub fn next_player(&self) -> Tile {
        match self {
            Tile::Circle => Tile::Cross,
            Tile::Cross => Tile::Circle,
            Tile::Empty => panic!("Empty tile does not have a next player"),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Tile::Empty => ".",
            Tile::Cross => "X",
            Tile::Circle => "O",
        };
        write!(f, "{}", symbol)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let board = Board::new();
        assert_eq!(board.tiles, [Tile::Empty; 9]);
    }
    #[test]
    #[should_panic]
    fn bad_set() {
        let mut board = Board::new();
        board.set(9, Tile::Cross);
    }
    #[test]
    fn set() {
        let mut board = Board::new();
        board.set(8, Tile::Cross);
        assert_eq!(board.tiles[8], Tile::Cross);
        board.set(7, Tile::Circle);
        assert_eq!(board.tiles[7], Tile::Circle);
    }
    #[test]
    fn check_win() {
        let winning_lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in winning_lines {
            let mut board = Board::new();
            for pos in line {
                board.set(pos, Tile::Cross);
            }
            assert!(
                board.check_win(Tile::Cross),
                "expected win for line {line:?}"
            );
            assert!(!board.check_win(Tile::Circle));
        }

        let board = Board::new();
        assert!(!board.check_win(Tile::Cross));
        assert!(!board.check_win(Tile::Circle));
    }
    #[test]
    fn check_empty_tile() {
        let board = Board::new();
        assert!(board.check_empty_tile(0));
    }
    #[test]
    fn check_full_board() {
        let mut board = Board::new();
        assert!(!board.check_full_board());
        for i in 0..9 {
            board.set(i, Tile::Circle);
        }
        assert!(board.check_full_board());
    }
}
