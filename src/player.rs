use crate::data::Position;
use crate::ship::{Direction, Ship, ShipType};

pub trait Player {
    fn new(height: usize, rows: &str) -> Self;
    fn create_ship(&self, ship_type: ShipType, height: usize, rows: &str) -> Ship;
    fn read_position(&self) -> Position;
    fn read_direction(&self) -> Direction;
    fn fire(&self) -> Position;
}
