#[test]
fn play() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    let mut game = match poker::Game::new(vec!["3357899k", "34668jq"], 1) {
        Ok(game) => game,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };
    game.play();
    game.print();
}
