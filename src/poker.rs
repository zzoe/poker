use crate::error::Error;
use crate::Error::Empty;
use indextree::{Arena, NodeId};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const CARD_DISPLAY: &str = "34567890JQKA2BR";
const THREE: u8 = 0;
const FOUR: u8 = 1;
const FIVE: u8 = 2;
const SIX: u8 = 3;
const SEVEN: u8 = 4;
const EIGHT: u8 = 5;
const NINE: u8 = 6;
const TEN: u8 = 7;
const JACK: u8 = 8;
const QUEEN: u8 = 9;
const KING: u8 = 10;
const ACE: u8 = 11;
const TWO: u8 = 12;
const BLACK_JOKER: u8 = 13;
const RED_JOKER: u8 = 14;

type Card = u8;

fn from_str(s: char) -> Result<Card, Error> {
    match s {
        '3' => Ok(THREE),
        '4' => Ok(FOUR),
        '5' => Ok(FIVE),
        '6' => Ok(SIX),
        '7' => Ok(SEVEN),
        '8' => Ok(EIGHT),
        '9' => Ok(NINE),
        '0' => Ok(TEN),
        'j' | 'J' => Ok(JACK),
        'q' | 'Q' => Ok(QUEEN),
        'k' | 'K' => Ok(KING),
        '1' | 'a' | 'A' => Ok(ACE),
        '2' => Ok(TWO),
        'b' | 'B' => Ok(BLACK_JOKER),
        'r' | 'R' => Ok(RED_JOKER),
        _ => Err(Error::InvalidCardValue(s.to_string())),
    }
}

/// 牌组
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    /// 不要
    None,
    /// 单张
    Single(Card),
    /// 五张顺子
    Straight5(Card),
    /// 六张顺子
    Straight6(Card),
    /// 七张顺子
    Straight7(Card),
    /// 八张顺子
    Straight8(Card),
    /// 九张顺子
    Straight9(Card),
    /// 十张顺子
    Straight10(Card),
    /// 十一张顺子
    Straight11(Card),
    /// 十二张顺子
    Straight12,
    /// 对子
    Pair(Card),
    /// 三连对
    PairStraight3(Card),
    /// 四连对
    PairStraight4(Card),
    /// 五连对
    PairStraight5(Card),
    /// 六连对
    PairStraight6(Card),
    /// 七连对
    PairStraight7(Card),
    /// 八连对
    PairStraight8(Card),
    /// 九连对
    PairStraight9(Card),
    /// 十连对
    PairStraight10(Card),
    /// 三张
    Triple(Card),
    /// 三带一单
    TripleSingle(Card, Card),
    /// 三带一对
    TriplePair(Card, Card),
    /// 飞机
    TripleStraight2(Card),
    /// 飞机带两单
    TripleStraight2Single(Card, Card, Card),
    /// 飞机带两双
    TripleStraight2Pair(Card, Card, Card),
    /// 三飞
    TripleStraight3(Card),
    /// 三飞带三单
    TripleStraight3Single(Card, Card, Card, Card),
    /// 三飞带三双
    TripleStraight3Pair(Card, Card, Card, Card),
    /// 四飞
    TripleStraight4(Card),
    /// 四飞带四单
    TripleStraight4Single(Card, Card, Card, Card, Card),
    /// 四飞带四双
    TripleStraight4Pair(Card, Card, Card, Card, Card),
    /// 五飞
    TripleStraight5(Card),
    /// 五飞带五单
    TripleStraight5Single(Card, Card, Card, Card, Card, Card),
    /// 六飞
    TripleStraight6(Card),
    /// 四带二单
    BombSingle(Card, Card, Card),
    /// 四带二对
    BombPair(Card, Card, Card),

    /// 炸弹
    Bomb(Card),
    /// 火箭
    Rocket,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Action::None);
        }
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Hand {
    kind: u16,
    amount: [u8; 15],
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hand = String::new();
        self.amount.iter().enumerate().for_each(|(i, count)| {
            let mut count = *count;
            while count > 0_u8 {
                hand += CARD_DISPLAY.get(i..i).unwrap();
                count -= 1;
            }
        });
        write!(f, "{}", hand)
    }
}

impl Hand {
    fn new(cards: &str) -> Result<Hand, Error> {
        if cards.is_empty() {
            return Err(Empty);
        }

        let mut hand = Hand::default();
        for c in cards.chars() {
            let card = from_str(c)?;
            hand.kind |= 1 << card;
            hand.amount[card as usize] += 1;
        }

        Ok(hand)
    }

    /// 出牌
    pub fn play(&self, action: &Action) -> Vec<(Action, Hand)> {
        self.single(action)
    }

    /// 出一张单牌
    fn single(&self, action: &Action) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();

        for i in 0..15 {
            let mut hand = *self;
            if self.kind & 1 << i > 0 {
                hand.amount[i] -= 1;
                if hand.amount[i] == 0 {
                    hand.kind ^= 1 << i;
                }
                actions.push((Action::Single(i as u8), hand));
            }
        }

        actions
    }
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
            hands.push(format!("{}", hand));
        }
        write!(
            f,
            "Action: {:?}, Hands: {:?}, Turn: {}, Pass: {}",
            self.action, hands, self.turn, self.pass
        )
    }
}

impl State {
    pub fn new(player_str: Vec<String>, turn: u8) -> Result<State, Error> {
        let mut player = Vec::new();
        for s in player_str {
            player.push(Hand::new(&*s)?);
        }

        Ok(State {
            action: Action::None,
            player,
            turn,
            pass: false,
        })
    }

    pub fn play(arena: &mut Arena<State>, node_id: NodeId) -> Option<NodeId> {
        let mut next_node_id = None;
        let state = arena.get(node_id).unwrap().get();
        let turn = state.turn;

        // player为空,说明是回溯回来的
        if state.player.is_empty() {
            if turn == 0 {
                let passed_child = node_id
                    .children(arena)
                    .find(|n| arena.get(*n).unwrap().get().pass);
                if passed_child.is_some() {
                    // 存在已经胜利的分支
                    arena.get_mut(node_id).unwrap().get_mut().pass = true;
                    // 删掉其它并列的分支
                    node_id
                        .children(arena)
                        .filter(|n| !arena.get(*n).unwrap().get().pass)
                        .collect::<Vec<NodeId>>()
                        .iter()
                        .for_each(|n| n.remove_subtree(arena));

                    return node_id.ancestors(arena).nth(1);
                }
            }

            let next_node_id = node_id
                .children(arena)
                .find(|n| !arena.get(*n).unwrap().get().pass);

            return if next_node_id.is_none() {
                arena.get_mut(node_id).unwrap().get_mut().pass = true;
                node_id.ancestors(arena).nth(1)
            } else {
                next_node_id
            };
        }

        if let Some(hand) = state.player.get(turn as usize) {
            let mut player = state.player.clone();
            for (action, hand) in hand.play(&state.action) {
                player[turn as usize] = hand;
                let pass = hand.kind == 0;

                let child = arena.new_node(State {
                    action,
                    player: player.clone(),
                    turn: (turn + 1) % player.len() as u8,
                    pass,
                });
                node_id.append(child, arena);

                if pass {
                    if turn != 0 {
                        // 其它玩家已经出光啦,此路不通
                        let last_node_id = node_id
                            .ancestors(arena)
                            .find(|n| arena.get(*n).unwrap().get().turn == 0)
                            .unwrap();
                        next_node_id = last_node_id.ancestors(arena).nth(1);
                        last_node_id.remove_subtree(arena);
                    } else {
                        // player1已经出光啦, 删除同层的其它出法
                        arena.get_mut(node_id).unwrap().get_mut().pass = true;
                        next_node_id = node_id.ancestors(arena).nth(1);
                        child
                            .preceding_siblings(arena)
                            .skip(1)
                            .collect::<Vec<NodeId>>()
                            .iter()
                            .for_each(|node| node.remove(arena));
                    }
                    break;
                } else if next_node_id.is_none() {
                    next_node_id = Some(child);
                }
            }
        }

        arena.get_mut(node_id).unwrap().get_mut().player = Vec::new();

        next_node_id
    }
}
