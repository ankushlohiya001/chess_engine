#![allow(dead_code)]

use std::mem;

use crate::{
    chess_board::ChessBoard,
    errors::GameError,
    pieces::{self, Character, Piece, Side},
    position::Pos,
};

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
    pub state: GameState,
    side: Side,
    captured_white: Vec<Character>,
    captured_black: Vec<Character>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: ChessBoard::new(),
            side: Side::White,
            state: GameState::Idle,
            captured_white: Vec::new(),
            captured_black: Vec::new(),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
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

    pub fn captured_pieces(&self, side: Side) -> &Vec<Character> {
        match side {
            Side::White => self.captured_white.as_ref(),
            Side::Black => self.captured_black.as_ref(),
        }
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

    pub fn promote_pawn(&mut self, pos: impl TryInto<Pos>) -> Result<(), GameError> {
        if let Ok(pos) = pos.try_into() {
            let mut piece = self.pick(pos)?;
            match pos.rank() {
                2 => {
                    if piece.side == Side::White {
                        piece.character = Character::Queen(piece.side);
                        piece.place_at(self, Pos(pos.file(), 8))
                    } else {
                        Err(GameError::InvalidMove)
                    }
                }
                7 => {
                    if piece.side == Side::Black {
                        piece.character = Character::Queen(piece.side);
                        piece.place_at(self, Pos(pos.file(), 8))
                    } else {
                        Err(GameError::InvalidMove)
                    }
                }
                _ => Err(GameError::InvalidMove),
            }
        } else {
            Err(GameError::InvalidPosition)
        }
    }

    pub fn en_passant_capture(&mut self, piece: Piece) {
        todo!("learn how to perform")
    }

    pub fn request_draw(&mut self) {
        todo!("to request draw")
    }

    pub fn resign(&mut self) {
        todo!("accepts defeat!")
    }

    pub fn is_game_over(&self) -> bool {
        todo!("provide way to tell this")
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
