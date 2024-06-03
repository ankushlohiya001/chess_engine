use chess_game_engine::{game::Game, moves::Moving};

fn main() {
    let mut game = Game::new();
    game.start();
    game.show_board();

    let pawn = game.pick("c2").unwrap();

    pawn.place_at(&mut game, "c3").unwrap();

    let pawn = game.pick("g7").unwrap();

    pawn.place_at(&mut game, "g5").unwrap();

    let bishop = game.pick("c1").unwrap();

    println!("{:?}", bishop.possible_moves());
    bishop.place_back(&mut game);

    let knight = game.pick("b1").unwrap();
    println!("{:?}", knight.possible_moves());
    knight.place_at(&mut game, "a3").unwrap();

    let piece = game.pick("g5").unwrap();
    piece.place_at(&mut game, "g4").unwrap();

    let piece = game.pick("h2").unwrap();
    piece.place_at(&mut game, "h3").unwrap();

    let piece = game.pick("g4").unwrap();
    //println!("{:?}", piece.possible_moves());
    piece.place_at(&mut game, "h3").unwrap();

    game.show_board();
}
