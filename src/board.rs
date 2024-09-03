use crate::data::{Cell, Position};
use crate::game::Response;
use crate::ship::Ship;
use crate::util::{char_to_num, get_poses};
use std::cell::RefCell;

pub struct Board {
    field: RefCell<Vec<Vec<Cell>>>,
    ships: RefCell<Vec<Ship>>,
    height: usize,
    width: usize,
    rows: String,
}

impl Board {
    pub fn new(height: usize, rows: String) -> Self {
        let width = rows.len();
        let field = RefCell::new(Self::create_field(height, rows.as_str()));
        let ships = RefCell::new(Vec::new());
        Self {
            field,
            height,
            width,
            rows,
            ships,
        }
    }

    pub fn add_ship(&self, ship: Ship) {
        let poses = get_poses(ship.coordinates());
        let len = self.ships.borrow().len();
        self.ships.borrow_mut().push(ship);
        for p in poses {
            let x = char_to_num(p.0);
            let y = p.1;
            self.field.borrow_mut()[y][x] = Cell::Busy(len);
        }
    }

    fn create_field(height: usize, str: &str) -> Vec<Vec<Cell>> {
        let mut field = Vec::new();
        for _ in 0..height {
            field.push(str.chars().map(|_| Cell::Empty).collect());
        }
        field
    }

    pub fn get_damage(&self, pos: Position) -> Response {
        let x = char_to_num(pos.0);
        let y = pos.1;

        let t = self.field.borrow()[y][x];
        match t {
            Cell::Busy(idx) => {
                self.ships.borrow_mut()[idx].get_damage();
                self.field.borrow_mut()[y][x] = Cell::Hit;
                Response::Hit
            }
            Cell::Empty => {
                self.field.borrow_mut()[y][x] = Cell::Missed;
                Response::Miss
            }
            Cell::Missed => Response::NotCorrect,
            Cell::Hit => Response::NotCorrect,
        }
    }

    pub fn ship_exist(&self) -> bool {
        for ship in self.ships.borrow().iter() {
            if ship.is_alive() {
                return true;
            }
        }
        false
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.field.borrow()[y][x]
    }

    pub fn rows(&self) -> &str {
        &self.rows
    }
}
