use std::fmt::{Display, Formatter};

use indextree::{Arena, NodeId};

use crate::action::Action;
use crate::card::Card;
use crate::hand::Hand;
use crate::Error;

pub(crate) enum StraightType {
    Single,
    Pair,
    Triple,
}

pub(crate) enum Carry {
    None,
    Single,
    Pair,
}

#[derive(Clone, Debug)]
pub struct State {
    /// 当前回合需要应对的牌
    action: Action,
    /// 玩家手牌
    player: Vec<Hand>,
    /// 当前回合谁出牌
    turn: u8,
    /// 当前方案验证已通过
    pass: bool,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hands = Vec::new();
        for hand in &self.player {
            hands.push(hand.to_string());
        }
        write!(
            f,
            "Action: {}, Hands: {:?}, Turn: {}, Pass: {}",
            self.action, hands, self.turn, self.pass
        )
    }
}

impl State {
    fn new(player_hand: Vec<impl Into<Hand>>, turn: u8) -> Result<State, Error> {
        let mut player = Vec::new();
        for s in player_hand {
            player.push(s.into());
        }

        Ok(State {
            action: Action::None,
            player,
            turn,
            pass: false,
        })
    }

    pub fn action_string(&self) -> String {
        self.action.to_string()
    }

    pub fn action_cards(&self) -> Vec<Card> {
        self.action.into()
    }
}

pub struct Game {
    pub arena: Arena<State>,
    pub root: NodeId,
}

impl Game {
    pub fn new(player_hand: Vec<impl Into<Hand>>, turn: u8) -> Result<Self, Error> {
        let mut arena = Arena::new();
        let root = arena.new_node(State::new(player_hand, turn)?);
        Ok(Game { arena, root })
    }

    pub fn pass(&self) -> bool {
        self.arena
            .get(self.root)
            .map(|n| !n.is_removed() && n.get().pass)
            .unwrap_or_default()
    }

    pub fn play(&mut self) {
        let mut next_node_id = Some(self.root);
        while let Some(node_id) = next_node_id {
            let state = self.arena.get(node_id).unwrap().get();
            next_node_id = if !state.player.is_empty() {
                // node_id: 当前节点有player
                self.expand(node_id)
            } else if state.turn == 0 {
                // node_id: 当前节点没有player, turn: 0
                self.expand_player1(node_id)
            } else {
                // node_id: 当前节点没有player, turn != 0
                self.expand_other_player(node_id)
            };
        }
    }

    fn expand_player1(&mut self, node_id: NodeId) -> Option<NodeId> {
        let children = node_id.children(&self.arena).collect::<Vec<NodeId>>();
        if children.is_empty() {
            // 没有其它分支，回退到上一次player1的分支
            return self.rollback_to_last_player1(node_id);
        }

        return if let Some(passed) = children
            .iter()
            .find(|&&n| self.arena.get(n).unwrap().get().pass)
        {
            self.delete_siblings(node_id, *passed)
        } else {
            Some(children[0])
        };
    }

    fn expand_other_player(&mut self, node_id: NodeId) -> Option<NodeId> {
        let todo = node_id
            .children(&self.arena)
            .find(|&n| !self.arena.get(n).unwrap().get().pass);
        if todo.is_none() {
            self.arena.get_mut(node_id).unwrap().get_mut().pass = true;
            node_id.ancestors(&self.arena).nth(1)
        } else {
            todo
        }
    }

    /// 展开下一级节点
    fn expand(&mut self, node_id: NodeId) -> Option<NodeId> {
        let mut state = self.arena.get(node_id)?.get().clone();
        let mut next_node_id = None;
        let turn = state.turn as usize;

        let Some(hand) = state.player.get(turn).filter(|&h|!h.is_empty()) else {
            log::error!("手牌为空？ {}", state);
            return None;
        };

        for (action, hand) in hand.follow(&state.action) {
            state.player[turn] = hand;
            let pass = hand.is_empty();
            let child = self.arena.new_node(State {
                action,
                player: state.player.clone(),
                turn: (state.turn + 1) % state.player.len() as u8,
                pass: pass && turn == 0,
            });
            node_id.append(child, &mut self.arena);
            log::trace!(
                "parent: {}, child:{}, node:{}",
                node_id,
                child,
                self.arena.get(child).unwrap().get()
            );

            if pass {
                self.arena.get_mut(node_id).unwrap().get_mut().player = Vec::new();
                return if turn != 0 {
                    self.rollback_to_last_player1(node_id)
                } else {
                    self.delete_siblings(node_id, child)
                };
            }

            if next_node_id.is_none() {
                next_node_id = Some(child);
            }
        }

        // if !node_id.is_removed(&self.arena) {
        self.arena.get_mut(node_id).unwrap().get_mut().player = Vec::new();
        // }

        next_node_id
    }

    /// 其它玩家已经出光啦,此路不通,找到player1的上一个Action
    fn rollback_to_last_player1(&mut self, node_id: NodeId) -> Option<NodeId> {
        let last_node_id = node_id
            .ancestors(&self.arena)
            .find(|n| self.arena.get(*n).unwrap().get().turn == 1)?;

        // 找到错误的Action的上一个节点，准备从此节点重新选择
        let next_node_id = last_node_id.ancestors(&self.arena).nth(1);
        // 删除错误的Action的子树
        log::trace!(
            "node_id: {}, delete subtree of: {}, next: {:?}",
            node_id,
            last_node_id,
            next_node_id
        );
        last_node_id.remove_subtree(&mut self.arena);
        next_node_id
    }

    /// player1已经出光啦, 删除同层的其它出法
    fn delete_siblings(
        &mut self,
        current_node_id: NodeId,
        passed_node_id: NodeId,
    ) -> Option<NodeId> {
        passed_node_id
            .preceding_siblings(&self.arena)
            .skip(1)
            .collect::<Vec<NodeId>>()
            .iter()
            .for_each(|n| {
                log::trace!("passed_node_id: {}, delete sibling: {}", passed_node_id, n);
                n.remove(&mut self.arena)
            });
        passed_node_id
            .following_siblings(&self.arena)
            .skip(1)
            .collect::<Vec<NodeId>>()
            .iter()
            .for_each(|n| {
                log::trace!("passed_node_id: {}, delete sibling: {}", passed_node_id, n);
                n.remove(&mut self.arena)
            });

        self.arena.get_mut(current_node_id).unwrap().get_mut().pass = true;
        current_node_id.ancestors(&self.arena).nth(1)
    }

    pub fn print(&self) {
        self.print_child(self.root, self.root)
    }

    fn print_child(&self, node_id: NodeId, parent_id: NodeId) {
        if node_id.is_removed(&self.arena) {
            return;
        }
        println!(
            "{} {}: {}",
            parent_id,
            node_id,
            self.arena.get(node_id).unwrap().get()
        );

        for child in node_id.children(&self.arena) {
            self.print_child(child, node_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_play() {
        let mut game = Game::new(vec!["123", "234"], 0).unwrap();
        game.play();
        game.print();
        assert!(game.pass());

        let mut game = Game::new(vec!["34", "5"], 0).unwrap();
        game.play();
        game.print();
        assert!(!game.pass());
    }
}
