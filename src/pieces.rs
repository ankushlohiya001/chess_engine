#![allow(dead_code)]

use std::{cell::RefCell, fmt::format, mem, ops::DerefMut};

use crate::{
    characters,
    chess_board::ChessBoard,
    errors::GameError,
    game::{Game, GameState},
    moves::Moving,
    position::Pos,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    White,
    Black,
}
impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Self::White => "White",
            Self::Black => "Black",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn is_white(&self) -> bool {
        self.side() == Side::White
    }

    pub fn is_black(&self) -> bool {
        !self.is_white()
    }

    pub fn symbol(&self) -> char {
        use characters::symbols::*;
        let symbol = match self {
            Self::King(_) => King,
            Self::Queen(_) => Queen,
            Self::Knight(_) => Knight,
            Self::Rook(_) => Rook,
            Self::Bishop(_) => Bishop,
            Self::Pawn(_) => Pawn,
        };
        if self.is_white() {
            symbol.0
        } else {
            symbol.1
        }
    }

    pub fn same_side(character_a: Character, character_b: Character) -> bool {
        character_a.side() == character_b.side()
    }
}

impl ToString for Character {
    fn to_string(&self) -> String {
        let character = match self {
            Self::King(_) => "King",
            Self::Queen(_) => "Queen",
            Self::Knight(_) => "Knight",
            Self::Bishop(_) => "Bishop",
            Self::Rook(_) => "Rook",
            Self::Pawn(_) => "Pawn",
        };

        let side = self.side().to_string();
        format!("{character}_{side}")
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub character: Character,
    pub position: Pos,
    pub side: Side,
    pub surrounding: Option<RefCell<ChessBoard>>,
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

    pub fn same_side(piece_a: &Piece, piece_b: &Piece) -> bool {
        piece_a.side == piece_b.side
    }

    pub fn place_at(
        self,
        game: &mut Game,
        pos: impl TryInto<Pos>,
    ) -> Result<Option<Character>, GameError> {
        match pos.try_into() {
            Ok(pos) => {
                let is_current_pos = self.position == pos; // want to place back
                if is_current_pos || self.can_move(pos) {
                    match self.surrounding {
                        Some(ref surrounding_ref) => {
                            let mut surrounding = surrounding_ref.borrow_mut();
                            let res = surrounding.place_character(self.character, pos);
                            if let Some(character) = res {
                                match game.whose_turn() {
                                    Side::White => game.captured_white.push(character),
                                    Side::Black => game.captured_black.push(character),
                                }
                            }
                            game.board = mem::take(surrounding.deref_mut());
                            if !is_current_pos {
                                game.state = GameState::PiecePlaced;
                                game.change_side(); //implicitly changing side
                                Ok(res)
                            } else {
                                Ok(None)
                            }
                        }
                        None => Err(GameError::AlonePiece),
                    }
                } else {
                    Err(GameError::InvalidMove)
                }
            }
            Err(_) => Err(GameError::InvalidPosition),
        }
    }

    pub fn place_back(self, game: &mut Game) {
        let position = self.position;
        self.place_at(game, position).unwrap();
    }
}

impl Moving for Piece {
    fn character(&self) -> Character {
        self.character
    }

    fn current_position(&self) -> Pos {
        self.position
    }

    fn surrounding(&self) -> std::cell::RefMut<'_, ChessBoard> {
        self.surrounding.as_ref().unwrap().borrow_mut()
    }
}

#[test]
fn piece_test() {
    let piece = Piece::new_alone(Character::King(Side::White), Pos('a', 1));

    assert_eq!(piece.side, Side::White);
    assert_eq!(piece.position, Pos('a', 1));

    let character = Character::Knight(Side::Black);

    assert_eq!(character.to_string(), "Knight_Black".to_owned());
}
