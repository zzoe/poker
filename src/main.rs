use crate::error::Error;
use crate::poker::Game;

pub mod error;
mod poker;

fn main() {
    if let Err(e) = play() {
        eprintln!("{}", e);
    }
}

fn play() -> Result<(), Error> {
    let mut game = Game::new(vec!["k9987533".to_string(), "qj866".to_string()], 0)?;
    game.play();
    game.print();

    Ok(())
}
