use crate::board::Board;
use crate::data::{Position, Positions};
use crate::display_board::DisplayBoard;
use crate::player::Player;
use crate::ship::{Ship, ShipType};
use std::cmp::{max, min};
use std::rc::Rc;

pub fn char_to_num(ch: char) -> usize {
    (ch as u32 - '0' as u32 - 49) as usize
}

pub fn get_poses(poses: Positions) -> Vec<Position> {
    let (pos1, pos2) = poses;
    let mut poses = Vec::new();

    if pos1.0 == pos2.0 {
        // ('a', 10), ('a', 12)
        for y in min(pos1.1, pos2.1)..=max(pos1.1, pos2.1) {
            poses.push(Position(pos1.0, y));
        }
    } else if pos1.1 == pos2.1 {
        // ('a', 10) ('c', 10)
        let x = pos1.1;
        for y in min(pos1.0, pos2.0)..=max(pos1.0, pos2.0) {
            poses.push(Position(y, x));
        }
    } else {
        panic!("Not expected error: Coordinates on diagonal")
    }
    poses
}

pub fn ship_in_bound(ship: &Ship, height: usize, rows: &str) -> bool {
    let (p1, p2) = ship.coordinates();
    if p1.1 >= height || !rows.contains(p1.0) {
        return false;
    }

    if p2.1 >= height || !rows.contains(p2.0) {
        return false;
    }
    true
}

pub fn intersection(pair1: Positions, pair2: Positions) -> bool {
    let poses1 = get_poses(pair1);
    let poses2 = get_poses(pair2);

    for p1 in poses1 {
        for p2 in poses2.iter() {
            if (p1.0 == p2.0 || p1.1 == p2.1) && Position::distance(p1, *p2) <= 1 {
                return true;
            }
        }
    }
    false
}

pub fn create_table(
    player: &impl Player,
    height: usize,
    rows: &str,
    allowed_ships: &Vec<ShipType>,
    draw: bool,
) -> Rc<Board> {
    let mut ships = Vec::new();

    let board = Rc::new(Board::new(height, rows.to_string()));

    let display = DisplayBoard::Player(board.clone());

    if draw {
        println!("{display}")
    }

    for ship_type in allowed_ships {
        loop {
            let ship = player.create_ship(*ship_type, height, rows);
            if right_position(&ship, &ships) {
                ships.push(ship.clone());
                board.add_ship(ship);
                break;
            } else {
                println!("Not correct position! Repeat input")
            }
        }
        if draw {
            println!("{display}")
        }
    }
    board
}

fn right_position(ship: &Ship, ships: &Vec<Ship>) -> bool {
    !ships
        .into_iter()
        .any(|ship1| intersection(ship.coordinates(), ship1.coordinates()))
}
