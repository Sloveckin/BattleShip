use crate::data::Position;
use crate::player::Player;
use crate::ship::{Direction, Ship, ShipType};
use crate::util::ship_in_bound;
use rand::Rng;

pub struct RandomPlayer {
    height: usize,
    rows: String,
}

impl Player for RandomPlayer {
    fn new(height: usize, rows: &str) -> Self {
        let rows = String::from(rows);
        Self { height, rows }
    }

    fn create_ship(&self, ship_type: ShipType, height: usize, rows: &str) -> Ship {
        loop {
            let position = self.read_position();
            let dir = self.read_direction();
            let ship = Ship::try_new(ship_type, position, dir);
            if ship.is_err() {
                continue;
            }
            let ship = ship.unwrap();
            if ship_in_bound(&ship, height, rows) {
                return ship;
            }
        }
    }

    fn read_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        let numb = rng.gen_range(0..self.height);
        let rand_idx = rng.gen_range(0..self.rows.len());
        let ch = self.rows.chars().nth(rand_idx).unwrap();
        Position(ch, numb)
    }

    fn read_direction(&self) -> Direction {
        let mut rng = rand::thread_rng();
        let numb = rng.gen_range(0..4);
        match numb {
            0 => Direction::Down,
            1 => Direction::Up,
            2 => Direction::Right,
            3 => Direction::Left,
            x => panic!("Not expected panic. value = {}", x),
        }
    }

    fn fire(&self) -> Position {
        self.read_position()
    }
}
