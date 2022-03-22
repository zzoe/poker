use indextree::Arena;

use crate::error::Error;
use crate::poker::State;

pub mod error;
mod poker;

fn main() {
    if let Err(e) = play() {
        eprintln!("{}", e);
    }
}

fn play() -> Result<(), Error> {
    let state = poker::State::new(vec!["123".to_string(), "234".to_string()], 0)?;

    let mut arena = Arena::new();
    let root = arena.new_node(state);
    let mut node_id = Some(root);

    while let Some(n) = node_id {
        node_id = State::play(&mut arena, n);
    }

    poker::print_arena(&arena, root, root);

    Ok(())
}
