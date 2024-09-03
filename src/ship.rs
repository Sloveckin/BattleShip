use crate::data::{Position, Positions};
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShipType {
    Battleship,
    Destroyer,
    Cruiser,
    Boat,
}

impl Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ShipType {
    pub fn size(&self) -> usize {
        match self {
            ShipType::Battleship => 4,
            ShipType::Destroyer => 2,
            ShipType::Cruiser => 3,
            ShipType::Boat => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ship {
    health: usize,
    coordinates: Positions,
    ship_type: ShipType,
}

impl Ship {
    pub fn new(ship_type: ShipType, pos: Position, dir: Direction) -> Self {
        let health = ship_type.size();
        let second_pos = pos.next_pos(dir, ship_type.size() - 1);
        let coordinates = (min(pos, second_pos), max(pos, second_pos));
        Ship {
            health,
            coordinates,
            ship_type,
        }
    }

    pub fn try_new(ship_type: ShipType, pos: Position, dir: Direction) -> Result<Self, ()> {
        let health = ship_type.size();
        let second_pos = pos.try_next_pos(dir, ship_type.size() - 1)?;
        let coordinates = (min(pos, second_pos), max(pos, second_pos));
        let ship = Ship {
            health,
            coordinates,
            ship_type,
        };
        Ok(ship)
    }

    pub fn coordinates(&self) -> Positions {
        self.coordinates
    }

    pub fn get_damage(&mut self) {
        self.health -= 1;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
