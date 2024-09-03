use crate::board::Board;
use crate::player::Player;
use std::cell::Cell;
use std::rc::Rc;

pub enum Response {
    Miss,
    Hit,
    NotCorrect,
}

#[derive(PartialEq)]
pub enum GameStatus {
    Continue,
    Stop,
}

pub struct Game<T: Player, S: Player> {
    first_player: T,
    second_player: S,

    first_board: Rc<Board>,
    second_board: Rc<Board>,
    round: Cell<usize>,
}

impl<T: Player, S: Player> Game<T, S> {
    pub fn new(
        first_player: T,
        first_board: Rc<Board>,
        second_player: S,
        second_board: Rc<Board>,
    ) -> Self {
        Self {
            first_player,
            second_player,
            first_board,
            second_board,
            round: Cell::new(0),
        }
    }

    pub fn play(&self) -> GameStatus {
        if self.round.get() % 2 == 0 {
            Self::make_move(&self.first_player, &self.second_board, 1);
            if !self.second_board.ship_exist() {
                println!("First player is win!");
                return GameStatus::Stop;
            }
        } else {
            Self::make_move(&self.second_player, &self.first_board, 2);
            if !self.first_board.ship_exist() {
                println!("Second player is win!");
                return GameStatus::Continue;
            }
        }
        self.round.set(self.round.get() + 1);
        GameStatus::Continue
    }

    fn make_move(player: &impl Player, board: &Board, index: usize) {
        loop {
            let mv = player.fire();
            let response = board.get_damage(mv);
            match response {
                Response::Miss => {
                    println!("{index} missed!");
                    break;
                }
                Response::Hit => {
                    println!("{index} hit!");
                    break;
                }
                Response::NotCorrect => {
                    println!("{index} repeat please..");
                }
            }
        }
    }
}
