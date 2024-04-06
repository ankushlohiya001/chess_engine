#![allow(non_upper_case_globals)]

pub mod symbols {
    pub const King: (char, char) = ('♔', '♚');
    pub const Queen: (char, char) = ('♕', '♛');
    pub const Rook: (char, char) = ('♖', '♜');
    pub const Bishop: (char, char) = ('♗', '♝');
    pub const Knight: (char, char) = ('♘', '♞');
    pub const Pawn: (char, char) = ('♙', '♟');
}

pub mod positions {
    use crate::position::Pos;

    pub const King: [Pos; 2] = [Pos('e', 1), Pos('e', 8)];
    pub const Queen: [Pos; 2] = [Pos('d', 1), Pos('d', 8)];
    pub const Bishop: [Pos; 4] = [Pos('c', 1), Pos('f', 1), Pos('c', 8), Pos('f', 8)];
    pub const Knight: [Pos; 4] = [Pos('b', 1), Pos('g', 1), Pos('b', 8), Pos('g', 8)];
    pub const Rook: [Pos; 4] = [Pos('a', 1), Pos('h', 1), Pos('a', 8), Pos('h', 8)];

    pub const Pawn: [Pos; 16] = [
        Pos('a', 2),
        Pos('b', 2),
        Pos('c', 2),
        Pos('d', 2),
        Pos('e', 2),
        Pos('f', 2),
        Pos('g', 2),
        Pos('h', 2),
        Pos('a', 7),
        Pos('b', 7),
        Pos('c', 7),
        Pos('d', 7),
        Pos('e', 7),
        Pos('f', 7),
        Pos('g', 7),
        Pos('h', 7),
    ];

    // pub const Pawn: [Pos; 16] = (0..16)
    //     .map(|x| Pos(('a' as u8 + x % 8) as char, if x > 7 { 7 } else { 1 }))
    //     .collect();
}

pub mod moves {
    use crate::moves::dirs::*;

    pub const King: [Dir; 8] = [
        TOP_LEFT, TOP, TOP_RIGHT, MID_LEFT, MID_RIGHT, BOT_LEFT, BOT, BOT_RIGHT,
    ];

    pub const Queen: [Dir; 8] = [
        TOP_LEFT, TOP, TOP_RIGHT, MID_LEFT, MID_RIGHT, BOT_LEFT, BOT, BOT_RIGHT,
    ];

    pub const Rook: [Dir; 4] = [TOP, MID_RIGHT, BOT, MID_LEFT];

    pub const Bishop: [Dir; 4] = [TOP_LEFT, TOP_RIGHT, BOT_RIGHT, BOT_LEFT];

    pub const Knight: [Dir; 8] = [
        TWO_TOP_LEFT,
        TWO_TOP_RIGHT,
        TWO_RIGHT_TOP,
        TWO_RIGHT_BOT,
        TWO_BOT_RIGHT,
        TWO_BOT_LEFT,
        TWO_LEFT_BOT,
        TWO_LEFT_TOP,
    ];

    pub const Pawn: [Dir; 3] = [TOP, ONE_TOP_LEFT, ONE_TOP_RIGHT];
}
