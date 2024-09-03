use crate::data::Position;
use crate::player::Player;
use crate::ship::{Direction, Ship, ShipType};
use crate::util::ship_in_bound;

pub struct InputPlayer {
    height: usize,
    rows: String,
}

impl Player for InputPlayer {
    fn new(height: usize, rows: &str) -> Self {
        let rows = String::from(rows);
        Self { height, rows }
    }

    fn create_ship(&self, ship_type: ShipType, height: usize, rows: &str) -> Ship {
        println!(
            "You have to place {} ship. Size = {} cells",
            ship_type,
            ship_type.size()
        );
        loop {
            let pos = self.read_position();
            let dir = self.read_direction();
            let res = Ship::new(ship_type, pos, dir);

            if ship_in_bound(&res, height, rows) {
                return res;
            }
            println!("Can't place ship with these coordinates. Repeat please")
        }
    }

    fn read_position(&self) -> Position {
        loop {
            println!("Write position:");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();
            let char = input1.chars().next();
            if char.is_none() {
                println!("Input error. Repeat please");
                continue;
            }
            let char = char.unwrap();

            if !self.rows.contains(char) {
                println!("Not correct position. Repeat please");
                continue;
            }

            if !char.is_alphabetic() {
                println!("Input error. Repeat please");
                continue;
            }

            let number = &input1[1..input1.len()].trim();
            let number = number.parse::<usize>();
            if number.is_err() {
                println!("Input error. Repeat please");
                continue;
            }

            let number = number.unwrap();
            if number >= self.height {
                println!("Not correct position. Repeat please");
                continue;
            }

            return Position(char, number);
        }
    }

    fn read_direction(&self) -> Direction {
        loop {
            println!("Write direction:");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            let dir = parse_dir(input2.trim());
            if dir.is_err() {
                println!("Input error. Repeat please");
                continue;
            }
            return dir.unwrap();
        }
    }

    fn fire(&self) -> Position {
        self.read_position()
    }
}

fn parse_dir(dir: &str) -> Result<Direction, ()> {
    if dir.eq("Up") {
        Ok(Direction::Up)
    } else if dir.eq("Down") {
        Ok(Direction::Down)
    } else if dir.eq("Right") {
        Ok(Direction::Right)
    } else if dir.eq("Left") {
        Ok(Direction::Left)
    } else {
        Err(())
    }
}
