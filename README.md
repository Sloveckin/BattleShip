# Battle_ship
Простой консольный морской бой, написанный на языке Rust. Игра останавливается тогда, когда у одного из двух игроков не остается кораблей.

## Dependency:
```toml
rand = "0.8"
```

## Players:

Для реализации логики игрока предоставлен трейт Player:
```rust
pub trait Player {
    fn new(height: usize, rows: &str) -> Self;
    fn create_ship(&self, ship_type: ShipType, height: usize, rows: &str) -> Ship;
    fn read_position(&self) -> Position;
    fn read_direction(&self) -> Direction;
    fn fire(&self) -> Position;
}
```

Также "из коробки" есть уже две имплементации этого трейта:
* InputPlayer - игрок, которым управляет человека
* RandomPlayer - простой бот, поведение которого рандомно

## Поле игрока в коде:
```rust
pub struct Board {
    field: RefCell<Vec<Vec<Cell>>>,
    ships: RefCell<Vec<Ship>>,
    height: usize,
    width: usize,
    rows: String,
}
```
## Поле игрока в консоли:

```bash
  abcdefghi
0 ........#
1 ####.....
2 .........
3 ......#..
4 #.....#..
5 #.....#..
6 .#..#....
7 ........#
8 ###.....#
```
