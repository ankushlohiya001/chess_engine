use crate::errors::GameError;
use std::fmt::Debug;
use std::ops::RangeInclusive;

const FILE_RANGE: RangeInclusive<u8> = ('a' as u8)..=('h' as u8);
const RANK_RANGE: RangeInclusive<u8> = 1..=8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos(pub char, pub u8);
impl Pos {
    pub fn new(file: char, rank: u8) -> Result<Pos, GameError> {
        if Pos::is_valid(file, rank) {
            Ok(Pos(file, rank))
        } else {
            Err(GameError::InvalidPosition)
        }
    }

    pub fn is_valid(file: char, rank: u8) -> bool {
        FILE_RANGE.contains(&(file as u8)) && RANK_RANGE.contains(&rank)
    }

    pub fn d_pos(&self, d_file: i32, d_rank: i32) -> Result<Pos, ()> {
        // need to refactor almost all stuff about this function
        let new_file = (self.file() as i32 + d_file) as u8 as char;
        let new_rank = (self.rank() as i32 + d_rank) as u8;
        if Pos::is_valid(new_file, new_rank) {
            Ok(Pos(new_file, new_rank))
        } else {
            Err(())
        }
    }

    pub fn rank(&self) -> u8 {
        self.1
    }

    pub fn file(&self) -> char {
        self.0
    }

    pub fn at_matrix(&self) -> (usize, usize) {
        (
            (8 - self.rank()) as usize,
            self.file() as usize - 'a' as usize,
        )
    }
}

impl TryFrom<&str> for Pos {
    type Error = GameError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_lowercase();
        Pos::new(
            s.chars().next().unwrap(),
            s.chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
        )
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
