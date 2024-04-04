use chess_game_engine::{game::Game, moves::Moving};

fn main() {
    let mut game = Game::new();
    game.start();
    game.show_board();

    let pawn = game.pick("c2").unwrap();

    pawn.place_at(&mut game, "c3").unwrap();

    game.change_side().unwrap();
    let pawn = game.pick("g7").unwrap();

    pawn.place_at(&mut game, "g5").unwrap();

    game.change_side().unwrap();

    let bishop = game.pick("c1").unwrap();

    println!("{:?}", bishop.possible_moves());
    bishop.place_back(&mut game);

    let knight = game.pick("b1").unwrap();
    println!("{:?}", knight.possible_moves());
    knight.place_at(&mut game, "a3");

    game.change_side().unwrap();

    let bishop = game.pick("f8").unwrap();
    println!("{:?}", bishop.possible_moves());
    bishop.place_at(&mut game, "h6");

    game.show_board();
}
