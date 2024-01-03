use std::ops::{Index, IndexMut, Neg};

mod display;

#[derive(Default, Copy, Clone)]
enum Cell {
    #[default]
    Empty,
    Unit(Player),
    Wall(Player),
}

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 10;

type Board = [[Cell; BOARD_WIDTH]; BOARD_HEIGHT];
type Mask = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum Player {
    #[default]
    Blue,
    Red,
}

struct Game {
    board: Board,
    player: Player,
    turns_left: usize,
}

impl Default for Game {
    fn default() -> Self {
        const TURNS_DEFAULT: usize = 3;

        let mut board = Board::default();

        board[0][0] = Cell::Unit(Player::Blue);
        board[BOARD_HEIGHT - 1][BOARD_WIDTH - 1] = Cell::Unit(Player::Red);

        Self {
            board,
            player: Player::default(),
            turns_left: TURNS_DEFAULT,
        }
    }
}

#[derive(Copy, Clone)]
struct BoardPos(usize, usize);

impl Index<BoardPos> for Board {
    type Output = Cell;

    fn index(&self, index: BoardPos) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl IndexMut<BoardPos> for Board {
    fn index_mut(&mut self, index: BoardPos) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

impl Neg for Player {
    type Output = Player;

    fn neg(self) -> Self::Output {
        match self {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        }
    }
}

impl Game {
    /// Returns collection of legal moves for current player.
    fn legal_moves(&self) -> Vec<BoardPos> {
        let mut processed = Mask::default();
        let mut moves = Vec::new();
        let mut stack = Vec::new();

        // Put all of our own units on the stack
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                match self.board[i][j] {
                    Cell::Unit(p) if p == self.player => {
                        stack.push(BoardPos(i, j));
                        processed[i][j] = true;
                    }
                    _ => {}
                }
            }
        }

        // Process tiles that are owned by us
        while let Some(BoardPos(i, j)) = stack.pop() {
            for (di, dj) in [
                (i.wrapping_sub(1), j.wrapping_sub(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_sub(1), j + 1),
                (i, j.wrapping_sub(1)),
                (i, j + 1),
                (i + 1, j.wrapping_sub(1)),
                (i + 1, j),
                (i + 1, j + 1),
            ] {
                if di >= BOARD_WIDTH || dj >= BOARD_HEIGHT {
                    continue;
                }
                if processed[di][dj] {
                    continue;
                }
                processed[di][dj] = true;

                match self.board[di][dj] {
                    Cell::Empty => moves.push(BoardPos(di, dj)),
                    Cell::Unit(p) => {
                        if p != self.player {
                            moves.push(BoardPos(di, dj));
                        }
                    }
                    Cell::Wall(p) => {
                        if p == self.player {
                            stack.push(BoardPos(di, dj));
                        }
                    }
                }
            }
        }

        moves
    }

    /// Recursively calculates the optimal move for the current player, until level reaches 0.
    /// Returns the board position that needs to be played in the current recursion and its evaluation.
    fn recurse(&mut self, level: usize) -> (BoardPos, isize) {
        let legal_moves = self.legal_moves();

        if level == 0 {
            self.player = -self.player;
            let opponent_legal_moves = self.legal_moves();
            self.player = -self.player;
            return (
                BoardPos(usize::MAX, usize::MAX),
                legal_moves.len() as isize - opponent_legal_moves.len() as isize,
            );
        }

        legal_moves
            .into_iter()
            .map(|mv| {
                self.do_move(mv);
                let (_, mut eval) = self.recurse(level - 1);
                self.undo_move(mv);

                if self.turns_left == 1 {
                    eval *= -1;
                }

                (mv, eval)
            })
            .max_by_key(|&(_, eval)| eval)
            .unwrap_or((BoardPos(usize::MAX, usize::MAX), isize::MIN))
    }

    fn do_move(&mut self, mv: BoardPos) {
        // Update tile.
        match self.board[mv] {
            Cell::Empty => self.board[mv] = Cell::Unit(self.player),
            Cell::Unit(_) => self.board[mv] = Cell::Wall(self.player),
            Cell::Wall(_) => unreachable!(),
        }

        // Update turns left and the player.
        if self.turns_left == 1 {
            self.turns_left = 3;
            self.player = -self.player;
        } else {
            self.turns_left -= 1;
        }
    }

    fn undo_move(&mut self, mv: BoardPos) {
        // Inverse of above.
        if self.turns_left == 3 {
            self.turns_left = 1;
            self.player = -self.player;
        } else {
            self.turns_left += 1;
        }

        match self.board[mv] {
            Cell::Empty => unreachable!(),
            Cell::Unit(_) => self.board[mv] = Cell::Empty,
            Cell::Wall(_) => self.board[mv] = Cell::Unit(-self.player),
        }
    }
}

fn main() {
    let mut game = Game::default();
    println!("{game}");

    let (pos, eval) = game.recurse(9);
    println!("Evaluation: {} / #{}", pos, eval);
}
