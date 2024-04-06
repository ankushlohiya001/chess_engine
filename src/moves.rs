use crate::{
    characters::moves,
    chess_board::ChessBoard,
    pieces::{Character, Side},
    position::Pos,
};

pub mod dirs {
    pub type Dir = (i32, i32);

    pub const TOP_LEFT: Dir = (-1, 1);
    pub const TOP: Dir = (0, 1);
    pub const TOP_RIGHT: Dir = (1, 1);

    pub const MID_LEFT: Dir = (-1, 0);
    pub const MID: Dir = (0, 0);
    pub const MID_RIGHT: Dir = (1, 0);

    pub const BOT_LEFT: Dir = (-1, -1);
    pub const BOT: Dir = (0, -1);
    pub const BOT_RIGHT: Dir = (1, -1);

    pub const TWO_TOP_LEFT: Dir = (-1, 2);
    pub const TWO_TOP_RIGHT: Dir = (1, 2);

    pub const TWO_LEFT_TOP: Dir = (-2, 1);
    pub const TWO_LEFT_BOT: Dir = (-2, -1);

    pub const TWO_RIGHT_TOP: Dir = (2, 1);
    pub const TWO_RIGHT_BOT: Dir = (2, -1);

    pub const TWO_BOT_LEFT: Dir = (-1, -2);
    pub const TWO_BOT_RIGHT: Dir = (1, -2);

    pub const ONE_TOP_LEFT: Dir = (-1, 1);
    pub const ONE_TOP_RIGHT: Dir = (1, 1);
}

// TODOs
// move manager can perform moves
// based on pattern specified in chess,
//  ie. e2, moving pawn to e2,
//      Kg4, moving King to g4
//
// also records so as to perform undos/redos
pub trait Moving {
    fn character(&self) -> Character;

    fn current_position(&self) -> Pos;

    fn surrounding(&self) -> std::cell::RefMut<'_, ChessBoard>;

    fn possible_moves(&self) -> Vec<Pos> {
        match self.character() {
            Character::Bishop(_) => {
                let dirs = moves::Bishop.to_vec();
                self.move_maker(dirs, true)
            }
            Character::Queen(_) => {
                let dirs = moves::Queen.to_vec();
                self.move_maker(dirs, true)
            }
            Character::Rook(_) => {
                let dirs = moves::Rook.to_vec();
                self.move_maker(dirs, true)
            }
            Character::Knight(_) => {
                let dirs = moves::Knight.to_vec();
                self.move_maker(dirs, false)
            }
            Character::King(_) => {
                let dirs = moves::King.to_vec();
                self.move_maker(dirs, false)
            }
            Character::Pawn(_) => {
                let Pos(_file, rank) = self.current_position();
                let first_move = rank == 2 || rank == 7;
                let dirs = moves::Pawn.to_vec();
                self.dirs_traverser(dirs, true, |cp, mc, (d_file, d_rank)| {
                    if d_file != 0 {
                        if let Some(other) = mc {
                            (!Character::same_side(cp, other), true)
                        } else {
                            (false, true)
                        }
                    } else {
                        (
                            mc.is_none() && (first_move || d_rank.abs() == 1),
                            (!first_move || d_rank.abs() == 2 || mc.is_some()),
                        )
                    }
                })
            }
        }
    }

    fn can_move(&self, new_pos: Pos) -> bool;

    fn general_condition(
        current_character: Character,
        maybe_character: Option<Character>,
        _pos: dirs::Dir,
    ) -> (bool, bool) {
        if let Some(nei) = maybe_character {
            (!Character::same_side(current_character, nei), true)
        } else {
            (true, false)
        }
    }

    fn move_maker(&self, dirs: Vec<dirs::Dir>, infinite: bool) -> Vec<Pos> {
        self.dirs_traverser(dirs, infinite, <Self as Moving>::general_condition)
    }

    fn dirs_traverser(
        &self,
        mut dirs: Vec<dirs::Dir>,
        infinite: bool,
        adding_condition: impl Fn(Character, Option<Character>, dirs::Dir) -> (bool, bool),
    ) -> Vec<Pos> {
        let mut moves = Vec::with_capacity(9);
        let pos = self.current_position();
        let surounding = self.surrounding();

        let max = if infinite { 8 } else { 1 };

        let di = if matches!(self.character().side(), Side::White) {
            1
        } else {
            -1
        };
        for i in 1..=max {
            let i = i * di;
            let mut to_remove = Vec::new();
            for (index, (d_file, d_rank)) in dirs.iter().enumerate() {
                if let Ok(pos) = pos.d_pos(d_file * i, d_rank * i) {
                    let maybe_character = surounding.character_at(pos);
                    let (is_valid_pos, stop_here) = adding_condition(
                        self.character(),
                        maybe_character,
                        (*d_file * i, *d_rank * i),
                    );
                    if is_valid_pos {
                        moves.push(pos);
                    }
                    if stop_here {
                        to_remove.push(index);
                    }
                } else {
                    to_remove.push(index);
                }
            }
            if infinite {
                to_remove.reverse();
                // just to preserve unneccasory iterations
                for index in to_remove {
                    dirs.remove(index);
                }
            }
        }

        moves
    }
}

#[test]
fn test_moves() {}
