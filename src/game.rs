use crate::board::Board;
use crate::board::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    Won(Tile),
    Draw,
}

pub struct Game {
    board: Board,
    current_turn: Tile,
    status: GameStatus,
}
impl Game {
    pub fn new(board: Board, start_tile: Tile) -> Game {
        if matches!(start_tile, Tile::Empty) {
            panic!("start_tile cannot be Empty")
        }
        let status = if board.check_win(start_tile) {
            GameStatus::Won(start_tile)
        } else if board.check_win(start_tile.next_player()) {
            GameStatus::Won(start_tile.next_player())
        } else if board.check_full_board() {
            GameStatus::Draw
        } else {
            GameStatus::Ongoing
        };
        Game {
            board,
            current_turn: start_tile,
            status,
        }
    }

    pub fn make_move(&mut self, pos: usize) {
        if pos > 8 {
            panic!("pos larger than 8")
        }

        self.board.set(pos, self.current_turn);
        if self.board.check_win(self.current_turn) {
            self.status = GameStatus::Won(self.current_turn);
        } else if self.board.check_full_board() {
            self.status = GameStatus::Draw;
        } else {
            self.current_turn = self.current_turn.next_player();
        }
    }
    pub fn status(&self) -> GameStatus {
        self.status
    }
    pub fn current_turn(&self) -> Tile {
        self.current_turn
    }
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn show_board(&self) {
        self.board.print_grid()
    }
}
impl Default for Game {
    fn default() -> Self {
        Self::new(Board::default(), Tile::Cross)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tile::Cross;
    #[test]
    #[should_panic]
    fn bad_constructor() {
        let _game = Game::new(Board::default(), Tile::Empty);
    }
    #[test]
    fn constructor() {
        let game = Game::default();
        assert_eq!(game.current_turn, Tile::Cross);
        assert_eq!(game.status(), GameStatus::Ongoing);
        assert_eq!(game.board.get_board(), [Tile::Empty; 9]);
    }
    #[test]
    fn make_move_basic() {
        let mut game = Game::default();
        assert_eq!(game.current_turn, Tile::Cross);
        game.make_move(0);
        assert_eq!(game.current_turn, Tile::Circle);
        assert_eq!(game.board.get_board()[0], Tile::Cross);

        game.make_move(1);
        assert_eq!(game.current_turn, Tile::Cross);
        assert_eq!(game.board.get_board()[1], Tile::Circle);
    }
    #[test]
    fn make_move_win() {
        let mut game = Game::default();
        // x o o
        // x - -
        // x - -
        game.make_move(0);
        game.make_move(1);
        game.make_move(3);
        game.make_move(2);
        game.make_move(6);
        assert_eq!(game.status, GameStatus::Won(Cross));
    }
    #[test]
    fn make_move_draw() {
        let mut game = Game::default();
        // x o x
        // x o o
        // o x x
        game.make_move(0);
        game.make_move(1);
        game.make_move(2);
        game.make_move(4);
        game.make_move(3);
        game.make_move(5);
        game.make_move(7);
        game.make_move(6);
        game.make_move(8);
        assert_eq!(game.status, GameStatus::Draw);
    }
    #[test]
    fn status() {
        let game = Game::default();
        assert_eq!(game.status(), GameStatus::Ongoing);
    }
    #[test]
    fn current_turn() {
        let game = Game::default();
        assert_eq!(game.current_turn(), Tile::Cross);
    }
    #[test]
    fn get_board() {
        let game = Game::default();
        assert_eq!(game.get_board().get_board(), [Tile::Empty; 9]);
    }
}
