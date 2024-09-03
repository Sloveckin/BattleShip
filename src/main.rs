use crate::display_board::DisplayBoard;
use crate::game::{Game, GameStatus};
use crate::input_player::InputPlayer;
use crate::player::Player;
use crate::random_player::RandomPlayer;
use crate::ship::ShipType;
use crate::ship::ShipType::{Battleship, Boat, Cruiser, Destroyer};
use crate::util::create_table;

mod board;
mod data;
mod display_board;
mod game;
mod input_player;
mod player;
mod random_player;
mod ship;
mod test;
mod util;

fn main() {
    let rows: String = String::from("abcdefghi");
    let height = 9;
    let allowed_ships: Vec<ShipType> = vec![
        Battleship, Cruiser, Cruiser, Destroyer, Destroyer, Boat, Boat, Boat,
    ];

    let input_player = InputPlayer::new(height, &rows);
    let enemy_player = RandomPlayer::new(height, &rows);

    let user_board = create_table(&input_player, height, &rows, &allowed_ships, true);
    let enemy_board = create_table(&enemy_player, height, &rows, &allowed_ships, false);

    let input_display = DisplayBoard::Player(user_board.clone());
    let enemy_display = DisplayBoard::Enemy(enemy_board.clone());

    display(&input_display, &enemy_display);
    let game = Game::new(input_player, user_board, enemy_player, enemy_board);

    while game.play() != GameStatus::Stop {
        display(&input_display, &enemy_display);
    }
}

fn display(d1: &DisplayBoard, d2: &DisplayBoard) {
    println!("First user:");
    println!("{d1}");
    println!();
    println!("Second user:");
    println!("{d2}");
}
