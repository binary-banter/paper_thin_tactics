use crate::{BoardPos, Cell, Game, Player};
use std::fmt::{Display, Formatter};

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
            Cell::Unit(Player::Blue) => write!(f, "{BOLD}{CYAN_FG}b{RESET}"),
            Cell::Unit(Player::Red) => write!(f, "{BOLD}{RED_FG}r{RESET}"),
            Cell::Wall(Player::Blue) => write!(f, "{BOLD}{CYAN_FG}B{RESET}"),
            Cell::Wall(Player::Red) => write!(f, "{BOLD}{RED_FG}R{RESET}"),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "It is the {} player's turn. {} turns are left.",
            self.player, self.turns_left
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

impl Display for BoardPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
