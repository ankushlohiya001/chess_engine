#![allow(dead_code)]

use std::mem;

use crate::chess_board::{ChessBoard, Pos};
use crate::errors::GameError;
use crate::moves::Moving;
use crate::pieces::Character;
use crate::pieces::{Piece, Side};

#[derive(Debug, Default)]
pub enum GameState {
    #[default]
    Idle,
    PiecePicked,
    PiecePlaced,
    Ended,
}

pub struct Game {
    pub board: ChessBoard,
    side: Side,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: ChessBoard::new(),
            side: Side::White,
            state: GameState::Idle,
        }
    }

    fn place_pieces(&mut self) {
        self.board.place_character_init();
    }

    pub fn start(&mut self) {
        self.start_with(Side::White);
    }

    pub fn start_with(&mut self, side: Side) {
        self.side = side;
        self.place_pieces();
    }

    pub fn whose_turn(&self) -> Side {
        self.side
    }

    pub fn show_board(&self) {
        self.board.show();
    }

    pub fn pick(&mut self, pos: impl TryInto<Pos>) -> Result<Piece, GameError> {
        // select a character / return an error

        match self.state {
            GameState::Idle => match pos.try_into() {
                Ok(pos) => match self.board.pick_character(pos) {
                    Ok(character) => {
                        let piece = Piece::new(character, pos, Some(mem::take(&mut self.board)));
                        if character.side() == self.side {
                            Ok(piece)
                        } else {
                            piece.place_back(self);
                            Err(GameError::OpponentPiece)
                        }
                    }
                    Err(_) => Err(GameError::EmptyCell),
                },
                Err(_) => Err(GameError::InvalidPosition),
            },
            _ => Err(GameError::SideNotChanged),
        }
    }

    pub fn change_side(&mut self) -> Result<(), GameError> {
        match self.state {
            GameState::PiecePlaced => {
                self.side = match self.side {
                    Side::White => Side::Black,
                    Side::Black => Side::White,
                };
                self.state = GameState::Idle;
                Ok(())
            }
            _ => Err(GameError::SideAlreadyChanged),
        }
    }

    pub fn castle(&mut self) {
        todo!("first learn rules of castling")
    }

    pub fn request_draw(&mut self) {
        todo!("to request draw")
    }

    pub fn promote_pawn(&mut self, piece: Piece) {
        todo!("apply rule of promotion")
    }

    pub fn en_passant_capture(&mut self, piece: Piece) {
        todo!("learn how to perform")
    }

    pub fn resign(&mut self) {
        todo!("accepts defeat!")
    }

    pub fn is_game_over(&self) -> bool {
        todo!("provide way to tell this")
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn game_test() {
    let mut game = Game::new();
    game.start();
    let res = game.change_side();
    assert_eq!(res, Err(GameError::SideAlreadyChanged));

    let whose_turn = game.whose_turn();
    assert_eq!(whose_turn, Side::White);

    let maybe_piece = game.pick("a1");
    if let Err(error) = maybe_piece {
        assert_eq!(error, GameError::EmptyCell);
    }
}

// somehow moves most piece related stuff to piece module,
