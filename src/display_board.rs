use crate::board::Board;
use crate::data::Cell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub enum DisplayBoard {
    Player(Rc<Board>),
    Enemy(Rc<Board>),
}

impl DisplayBoard {
    fn symbol(&self, cell: Cell) -> char {
        match self {
            DisplayBoard::Player(_) => match cell {
                Cell::Busy(_) => '#',
                Cell::Empty => '.',
                Cell::Hit => 'x',
                Cell::Missed => '0',
            },
            DisplayBoard::Enemy(_) => match cell {
                Cell::Busy(_) => '.',
                Cell::Empty => '.',
                Cell::Hit => 'x',
                Cell::Missed => '0',
            },
        }
    }
}

impl<'a> Display for DisplayBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let board = match self {
            DisplayBoard::Player(x) => x,
            DisplayBoard::Enemy(x) => x,
        };
        let mut result = String::with_capacity(
            board.width() * board.height() + board.width() + 2 * board.height(),
        );
        result.push_str("  ");
        for ch in board.rows().chars() {
            result.push(ch);
        }
        result.push('\n');
        for y in 0..board.height() {
            result.push_str(y.to_string().as_str());
            result.push(' ');

            for x in 0..board.width() {
                result.push(self.symbol(board.get_cell(x, y)));
            }

            result.push('\n');
        }
        write!(f, "{result}")
    }
}
