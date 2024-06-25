use std::fmt::{Display, Formatter};

use crate::card::Card;
use crate::game::{Carry, StraightType};

/// 牌组
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Action {
    /// 不要
    None,
    /// 单张
    Single(Card),
    /// 五张顺子
    Sequence5(Card),
    /// 六张顺子
    Sequence6(Card),
    /// 七张顺子
    Sequence7(Card),
    /// 八张顺子
    Sequence8(Card),
    /// 九张顺子
    Sequence9(Card),
    /// 十张顺子
    Sequence10(Card),
    /// 十一张顺子
    Sequence11(Card),
    /// 十二张顺子
    Sequence12,
    /// 对子
    Pair(Card),
    /// 三连对
    PairSequence3(Card),
    /// 四连对
    PairSequence4(Card),
    /// 五连对
    PairSequence5(Card),
    /// 六连对
    PairSequence6(Card),
    /// 七连对
    PairSequence7(Card),
    /// 八连对
    PairSequence8(Card),
    /// 九连对
    PairSequence9(Card),
    /// 十连对
    PairSequence10(Card),
    /// 三张
    Triplet(Card),
    /// 三带一单
    TripletSingle(Card, Card),
    /// 三带一对
    TripletPair(Card, Card),
    /// 飞机
    TripletSequence2(Card),
    /// 飞机带两单
    TripletSequence2Single(Card, Card, Card),
    /// 飞机带两双
    TripletSequence2Pair(Card, Card, Card),
    /// 三飞
    TripletSequence3(Card),
    /// 三飞带三单
    TripletSequence3Single(Card, Card, Card, Card),
    /// 三飞带三双
    TripletSequence3Pair(Card, Card, Card, Card),
    /// 四飞
    TripletSequence4(Card),
    /// 四飞带四单
    TripletSequence4Single(Card, Card, Card, Card, Card),
    /// 四飞带四双
    TripletSequence4Pair(Card, Card, Card, Card, Card),
    /// 五飞
    TripletSequence5(Card),
    /// 五飞带五单
    TripletSequence5Single(Card, Card, Card, Card, Card, Card),
    /// 六飞
    TripletSequence6(Card),
    /// 四带二单
    QuadSingle(Card, Card, Card),
    /// 四带二对
    QuadPair(Card, Card, Card),

    /// 炸弹
    Bomb(Card),
    /// 火箭
    Rocket,
}

impl From<Action> for Vec<Card> {
    fn from(action: Action) -> Self {
        match action {
            Action::None => Vec::new(),
            Action::Single(c) => vec![c],
            Action::Sequence5(c) => {
                straight_cards(StraightType::Single, c, 5, Carry::None, Vec::new())
            }
            Action::Sequence6(c) => {
                straight_cards(StraightType::Single, c, 6, Carry::None, Vec::new())
            }
            Action::Sequence7(c) => {
                straight_cards(StraightType::Single, c, 7, Carry::None, Vec::new())
            }
            Action::Sequence8(c) => {
                straight_cards(StraightType::Single, c, 8, Carry::None, Vec::new())
            }
            Action::Sequence9(c) => {
                straight_cards(StraightType::Single, c, 9, Carry::None, Vec::new())
            }
            Action::Sequence10(c) => {
                straight_cards(StraightType::Single, c, 10, Carry::None, Vec::new())
            }
            Action::Sequence11(c) => {
                straight_cards(StraightType::Single, c, 11, Carry::None, Vec::new())
            }
            Action::Sequence12 => straight_cards(
                StraightType::Single,
                Card::Three,
                12,
                Carry::None,
                Vec::new(),
            ),
            Action::Pair(c) => straight_cards(StraightType::Pair, c, 1, Carry::None, Vec::new()),
            Action::PairSequence3(c) => {
                straight_cards(StraightType::Pair, c, 3, Carry::None, Vec::new())
            }
            Action::PairSequence4(c) => {
                straight_cards(StraightType::Pair, c, 4, Carry::None, Vec::new())
            }
            Action::PairSequence5(c) => {
                straight_cards(StraightType::Pair, c, 5, Carry::None, Vec::new())
            }
            Action::PairSequence6(c) => {
                straight_cards(StraightType::Pair, c, 6, Carry::None, Vec::new())
            }
            Action::PairSequence7(c) => {
                straight_cards(StraightType::Pair, c, 7, Carry::None, Vec::new())
            }
            Action::PairSequence8(c) => {
                straight_cards(StraightType::Pair, c, 8, Carry::None, Vec::new())
            }
            Action::PairSequence9(c) => {
                straight_cards(StraightType::Pair, c, 9, Carry::None, Vec::new())
            }
            Action::PairSequence10(c) => {
                straight_cards(StraightType::Pair, c, 10, Carry::None, Vec::new())
            }
            Action::Triplet(c) => {
                straight_cards(StraightType::Triple, c, 1, Carry::None, Vec::new())
            }
            Action::TripletSingle(c1, c2) => {
                straight_cards(StraightType::Triple, c1, 1, Carry::Single, vec![c2])
            }
            Action::TripletPair(c1, c2) => {
                straight_cards(StraightType::Triple, c1, 1, Carry::Pair, vec![c2])
            }
            Action::TripletSequence2(c) => {
                straight_cards(StraightType::Triple, c, 2, Carry::None, Vec::new())
            }
            Action::TripletSequence2Single(c1, c2, c3) => {
                straight_cards(StraightType::Triple, c1, 2, Carry::Single, vec![c2, c3])
            }
            Action::TripletSequence2Pair(c1, c2, c3) => {
                straight_cards(StraightType::Triple, c1, 2, Carry::Pair, vec![c2, c3])
            }
            Action::TripletSequence3(c) => {
                straight_cards(StraightType::Triple, c, 3, Carry::None, Vec::new())
            }
            Action::TripletSequence3Single(c1, c2, c3, c4) => {
                straight_cards(StraightType::Triple, c1, 3, Carry::Single, vec![c2, c3, c4])
            }
            Action::TripletSequence3Pair(c1, c2, c3, c4) => {
                straight_cards(StraightType::Triple, c1, 3, Carry::Pair, vec![c2, c3, c4])
            }
            Action::TripletSequence4(c) => {
                straight_cards(StraightType::Triple, c, 4, Carry::None, Vec::new())
            }
            Action::TripletSequence4Single(c1, c2, c3, c4, c5) => straight_cards(
                StraightType::Triple,
                c1,
                4,
                Carry::Single,
                vec![c2, c3, c4, c5],
            ),
            Action::TripletSequence4Pair(c1, c2, c3, c4, c5) => straight_cards(
                StraightType::Triple,
                c1,
                4,
                Carry::Pair,
                vec![c2, c3, c4, c5],
            ),
            Action::TripletSequence5(c) => {
                straight_cards(StraightType::Triple, c, 5, Carry::None, Vec::new())
            }
            Action::TripletSequence5Single(c1, c2, c3, c4, c5, c6) => straight_cards(
                StraightType::Triple,
                c1,
                5,
                Carry::Single,
                vec![c2, c3, c4, c5, c6],
            ),
            Action::TripletSequence6(c) => {
                straight_cards(StraightType::Triple, c, 6, Carry::None, Vec::new())
            }
            Action::QuadSingle(c1, c2, c3) => vec![c1, c1, c1, c1, c2, c3],
            Action::QuadPair(c1, c2, c3) => vec![c1, c1, c1, c1, c2, c2, c3, c3],
            Action::Bomb(c) => vec![c, c, c, c],
            Action::Rocket => vec![Card::BlackJoker, Card::RedJoker],
        }
    }
}

fn straight_cards(
    st: StraightType,
    card: Card,
    length: u8,
    carry: Carry,
    carry_cards: Vec<Card>,
) -> Vec<Card> {
    let mut cards = Vec::new();
    let mut card = card;
    let count = match st {
        StraightType::Single => 1_u8,
        StraightType::Pair => 2,
        StraightType::Triple => 3,
    };

    for _ in 0..length {
        for _ in 0..count {
            cards.push(card);
        }
        card = card.plus().unwrap();
    }

    match carry {
        Carry::None => {}
        Carry::Single => {
            for c in carry_cards {
                cards.push(c);
            }
        }
        Carry::Pair => {
            for c in carry_cards {
                cards.push(c);
                cards.push(c);
            }
        }
    }
    cards
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a: Vec<Card> = (*self).into();
        if a.is_empty() {
            return write!(f, "不要");
        }
        let s = a
            .iter()
            .map(|c| c.to_string())
            .fold(String::new(), |mut a, b| {
                a.push_str(&b);
                a
            });
        write!(f, "{s}")
    }
}
