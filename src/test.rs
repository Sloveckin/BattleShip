#[cfg(test)]
pub mod test {
    use crate::util::char_to_num;

    use crate::board::{Board, Position};
    use crate::intersection;
    use crate::ship::ShipType::{Battleship, Destroyer};
    use crate::ship::{Direction, Ship};

    #[test]
    pub fn test1() {
        let p1 = Position('a', 2);
        let p2 = Position('a', 3);

        let p11 = Position('b', 3);

        let p12 = Position('b', 4);
        let p22 = Position('b', 5);

        assert_eq!(intersection((p1, p2), (p11, p22)), true);
        assert_eq!(intersection((p1, p2), (p12, p22)), false);
    }

    #[test]
    pub fn test2() {
        assert_eq!(char_to_num('a'), 0);
        assert_eq!(char_to_num('b'), 1);
        assert_eq!(char_to_num('c'), 2);
        assert_eq!(char_to_num('d'), 3);
    }

    #[test]
    pub fn test3() {
        let p1 = Position('a', 0);
        let p2 = p1.next_pos(Direction::Right, 3);

        let p11 = Position('c', 0);
        let p22 = p11.next_pos(Direction::Right, 3);
        assert_eq!(intersection((p1, p2), (p11, p22)), true);
    }

    #[test]
    pub fn test4() {
        // abcd
        let p1 = Position('d', 3);
        let p2 = Position('a', 0);
        assert_eq!(p1.next_pos(Direction::Left, 2), Position('b', 3));
        assert_eq!(p2.next_pos(Direction::Right, 1), Position('b', 0));
    }

    #[test]
    pub fn test5() {
        let p1 = Position('f', 0);
        let p2 = Position('f', 2);

        let p11 = Position('a', 5);
        let p22 = Position('b', 5);

        assert_eq!(intersection((p1, p2), (p11, p22)), false);
        assert_eq!(intersection((p11, p22), (p1, p2)), false);
    }

    #[test]
    pub fn test6() {
        let ship = Ship::new(Battleship, Position('a', 0), Direction::Right);
        let ship1 = Ship::new(Destroyer, Position('c', 1), Direction::Down);
        let board = Board::new(4, "abcd".to_string());
        board.add_ship(ship);
        board.add_ship(ship1);
    }
}
