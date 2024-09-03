use crate::ship::Direction;
use crate::util::char_to_num;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position(pub char, pub usize);

impl Position {
    pub fn next_pos(&self, dir: Direction, step: usize) -> Self {
        match dir {
            Direction::Up => Position(self.0, self.1 - step),
            Direction::Down => Position(self.0, self.1 + step),
            Direction::Right => Position(
                std::char::from_u32(self.0 as u32 + step as u32).unwrap(),
                self.1,
            ),
            Direction::Left => Position(
                std::char::from_u32(self.0 as u32 - step as u32).unwrap(),
                self.1,
            ),
        }
    }

    pub fn try_next_pos(&self, dir: Direction, step: usize) -> Result<Position, ()> {
        match dir {
            Direction::Left => {
                if step > char_to_num(self.0) {
                    Err(())
                } else {
                    Ok(self.next_pos(dir, step))
                }
            }
            Direction::Up => {
                if step > self.1 {
                    Err(())
                } else {
                    Ok(self.next_pos(dir, step))
                }
            }
            _ => Ok(self.next_pos(dir, step)),
        }
    }

    pub fn distance(p1: Position, p2: Position) -> usize {
        if p1.1 == p2.1 {
            let f = p1.0 as u32;
            let s = p2.0 as u32;
            f.abs_diff(s) as usize
        } else if p1.0 == p2.0 {
            p1.1.abs_diff(p2.1)
        } else {
            panic!("Positions on diagonal");
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Busy(usize),
    Missed,
    Empty,
    Hit,
}

pub type Positions = (Position, Position);
