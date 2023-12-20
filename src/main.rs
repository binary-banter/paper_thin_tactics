use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default)]
enum Cell {
    #[default]
    Empty,
    Blue,
    Red,
    BlueWall,
    RedWall,
}

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 10;

type Board = [[Cell; BOARD_WIDTH]; BOARD_HEIGHT];
type Mask = [[bool; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Default)]
enum Player {
    #[default]
    Blue,
    Red,
}

struct Game {
    board: Board,
    player: Player,
    turns: usize,
    legal_blue: Mask,
    legal_red: Mask,
}

impl Default for Game {
    fn default() -> Self {
        const TURNS_DEFAULT: usize = 3;

        let mut board = Board::default();

        board[0][0] = Cell::Blue;
        board[BOARD_HEIGHT - 1][BOARD_WIDTH - 1] = Cell::Red;

        let mut legal_blue = Mask::default();

        legal_blue[0][1] = true;
        legal_blue[1][0] = true;
        legal_blue[1][1] = true;

        let mut legal_red = Mask::default();

        legal_red[BOARD_HEIGHT - 2][BOARD_WIDTH - 2] = true;
        legal_red[BOARD_HEIGHT - 2][BOARD_WIDTH - 1] = true;
        legal_red[BOARD_HEIGHT - 1][BOARD_WIDTH - 2] = true;

        Self {
            board,
            player: Player::default(),
            turns: TURNS_DEFAULT,
            legal_blue,
            legal_red,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "It is the {} player's turn. {} turns are left.",
            self.player, self.turns
        )?;
        for row in self.board {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

const RESET: &str = "\x1B[0m";
const CYAN_FG: &str = "\x1B[36m";
const RED_FG: &str = "\x1B[31m";
const BOLD: &str = "\x1B[1m";

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Blue => write!(f, "{BOLD}{CYAN_FG}Blue{RESET}"),
            Player::Red => write!(f, "{BOLD}{RED_FG}Red{RESET}"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "{BOLD}."),
            Cell::Blue => write!(f, "{BOLD}{CYAN_FG}b{RESET}"),
            Cell::Red => write!(f, "{BOLD}{RED_FG}r{RESET}"),
            Cell::BlueWall => write!(f, "{BOLD}{CYAN_FG}B{RESET}"),
            Cell::RedWall => write!(f, "{BOLD}{RED_FG}R{RESET}"),
        }
    }
}

impl Game {
    /// Place a cell. Returns whether the placement was legal.
    fn place(&mut self, y: usize, x: usize) -> bool {
        let legal = match self.player {
            Player::Blue => self.legal_blue,
            Player::Red => self.legal_red,
        };

        if legal[y][x] {
            self.board[y][x] = match self.board[y][x] {
                Cell::Empty => match self.player {
                    Player::Blue => Cell::Red,
                    Player::Red => Cell::Blue,
                },
                Cell::Blue => Cell::RedWall,
                Cell::Red => Cell::BlueWall,
                Cell::BlueWall | Cell::RedWall => unreachable!(),
            };

            self.update_legal(y, x);

            true
        } else {
            false
        }
    }

    /// Should be called whenever a cell is placed to update legal moves.
    /// Returns whether the next player has legal moves left.
    fn update_legal(&mut self, y: usize, x: usize) -> bool {
        todo!()
    }
}

fn main() {
    let game = Game::default();
    println!("{game}");
}
