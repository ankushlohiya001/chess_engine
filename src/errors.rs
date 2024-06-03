#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    EmptyCell,
    SideNotChanged,
    SideAlreadyChanged,
    OpponentPiece,
    InvalidMove,
    GameOver,
    InvalidPosition,
    AlonePiece,
}

pub enum PosErr {
    InvalidPosition,
    ParseError,
}
