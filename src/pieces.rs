#![allow(dead_code)]

use std::{cell::RefCell, mem, ops::DerefMut};

use crate::{
    chess_board::ChessBoard,
    errors::GameError,
    game::{self, Game},
    moves::{cross_move, one_two_move, plus_move, two_n_half_move, Moving, Pos},
    pieces,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Character {
    King(Side),
    Queen(Side),
    Knight(Side),
    Rook(Side),
    Bishop(Side),
    Pawn(Side),
}

impl Character {
    pub fn side(self) -> Side {
        match self {
            Self::King(side) => side,
            Self::Queen(side) => side,
            Self::Knight(side) => side,
            Self::Rook(side) => side,
            Self::Bishop(side) => side,
            Self::Pawn(side) => side,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub character: Character,
    pub position: Pos,
    pub side: Side,
    surrounding: Option<RefCell<ChessBoard>>,
}

impl Piece {
    pub fn new(character: Character, position: Pos, surrounding: Option<ChessBoard>) -> Self {
        let side = character.side();
        Piece {
            character,
            position,
            side,
            surrounding: surrounding.map(RefCell::new),
        }
    }

    pub fn new_alone(character: Character, position: Pos) -> Self {
        Self::new(character, position, None)
    }

    pub fn place_at(&self, game: &mut Game, file: char, rank: u8) -> Result<(), GameError> {
        let pos = Pos(file, rank);
        if self.can_move(pos) {
            if let Some(ref surrounding_ref) = self.surrounding {
                let mut surrounding = surrounding_ref.borrow_mut();
                let res = surrounding.place_character(self.character, pos);
                game.board = mem::take(surrounding.deref_mut());
                res
            } else {
                Err(GameError::AlonePiece)
            }
        } else {
            Err(GameError::InvalidMove)
        }
    }

    pub fn place_back(&self, game: &mut Game) {
        let pos = self.position;
        self.place_at(game, pos.file(), pos.rank()).unwrap();
    }
}

impl Moving for Piece {
    fn possible_moves(&self) -> Vec<Pos> {
        match self.character {
            Character::Bishop(_) => cross_move(self.position, true),
            Character::Rook(_) => plus_move(self.position, true),
            Character::King(_) => {
                let cross_s = cross_move(self.position, false);
                let plus_s = plus_move(self.position, false);
                [cross_s, plus_s].concat()
            }
            Character::Queen(_) => {
                let cross_s = cross_move(self.position, true);
                let plus_s = plus_move(self.position, true);
                [cross_s, plus_s].concat()
            }
            Character::Knight(_) => two_n_half_move(self.position),
            Character::Pawn(_) => one_two_move(self.position),
        }
    }

    fn can_move(&self, new_pos: Pos) -> bool {
        self.possible_moves().contains(&new_pos)
    }
}

#[test]
fn piece_test() {
    let piece = Piece::new_alone(Character::King(Side::White), Pos('a', 1));

    assert_eq!(piece.side, Side::White);
    assert_eq!(piece.position, Pos('a', 1));

    assert!(piece.can_move(Pos('b', 2)));
}
