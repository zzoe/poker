use std::io::Write;

use anyhow::Result;
use poker::Game;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    if let Err(e) = interactive() {
        log::error!("{}", e);
    }
}

fn interactive() -> Result<()> {
    loop {
        let hand_own = read("请输入自己的手牌\n");
        let hand_opponent = read("请输入对手的手牌\n");
        let turn = read("请选择先手： 0-自己先手  1-对方先手； 默认0-自己先手\n")
            .parse::<u8>()
            .map(|t| t != 0)
            .unwrap_or_default();

        let game = match poker::Game::new(
            vec![hand_own.as_str(), hand_opponent.as_str()],
            turn.then(|| 1).unwrap_or_default(),
        ) {
            Ok(game) => game,
            Err(e) => {
                log::error!("创建游戏失败： {}", e);
                continue;
            }
        };

        if game.pass() {
            std::io::stdout().write_all("有必胜的方案\n".as_ref())?;
            play(game, turn);
        } else {
            std::io::stdout().write_all("没有必胜的方案\n".as_ref())?;
        };
    }
}

fn read(hint: &str) -> String {
    loop {
        match read_inner(hint) {
            Ok(s) => {
                return s;
            }
            Err(e) => {
                std::io::stdout()
                    .write_all(format!("请重新输入! {}", e).as_bytes())
                    .ok();
            }
        }
    }
}

fn read_inner(hint: &str) -> Result<String> {
    std::io::stdout().write_all(hint.as_ref())?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    buffer = buffer.trim_end().to_string();
    Ok(buffer)
}

fn play(game: Game, init_turn: bool) {
    let mut node_id = game.root;
    let mut turn = init_turn;
    loop {
        if !turn {
            if let Some(n) = node_id.children(&game.arena).next() {
                node_id = n;
            }
            let state = game.arena.get(node_id).unwrap().get();
            std::io::stdout()
                .write_all(format!("我方出牌：{}\n", state.action_string()).as_ref())
                .ok();
        } else {
            let actions = node_id
                .children(&game.arena)
                .map(|n| game.arena.get(n).unwrap().get().action_string())
                .collect::<Vec<_>>();

            if actions.is_empty() {
                std::io::stdout().write_all("胜利！\n\n".as_ref()).ok();
                return;
            }

            let mut action = read(&*format!(
                "{:?}\n请输入对方的出牌：(retract-悔一步 retry-重来 new-下一局 quit-退出)\n",
                actions
            ))
            .to_uppercase();

            if action.is_empty() {
                action = "不要".to_string();
            }

            match &*action {
                "RETRACT" => {
                    if let Some(n) = node_id.ancestors(&game.arena).nth(2) {
                        node_id = n;
                        continue;
                    }
                }
                "RETRY" => {
                    node_id = game.root;
                    turn = init_turn;
                    continue;
                }
                "NEW" => return,
                "QUIT" => std::process::exit(0),
                _ => {}
            }

            let Some(n) = node_id.children(&game.arena).find(|child| game.arena.get(*child).unwrap().get().action_string().eq(&action)) else
            {
                std::io::stdout().write_all("无效的出牌！\n".as_ref()).ok();
                continue;
            };
            node_id = n;
        }
        turn = !turn;
    }
}
