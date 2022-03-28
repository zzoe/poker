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

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Action::None => "不要".to_string(),
            Action::Single(c) => c.to_string(),
            Action::Straight5(c) => {
                straight_to_string(StraightType::Single, c, 5, Carry::None, Vec::new())
            }
            Action::Straight6(c) => {
                straight_to_string(StraightType::Single, c, 6, Carry::None, Vec::new())
            }
            Action::Straight7(c) => {
                straight_to_string(StraightType::Single, c, 7, Carry::None, Vec::new())
            }
            Action::Straight8(c) => {
                straight_to_string(StraightType::Single, c, 8, Carry::None, Vec::new())
            }
            Action::Straight9(c) => {
                straight_to_string(StraightType::Single, c, 9, Carry::None, Vec::new())
            }
            Action::Straight10(c) => {
                straight_to_string(StraightType::Single, c, 10, Carry::None, Vec::new())
            }
            Action::Straight11(c) => {
                straight_to_string(StraightType::Single, c, 11, Carry::None, Vec::new())
            }
            Action::Straight12 => straight_to_string(
                StraightType::Single,
                &Card::Three,
                12,
                Carry::None,
                Vec::new(),
            ),
            Action::Pair(c) => {
                straight_to_string(StraightType::Pair, c, 1, Carry::None, Vec::new())
            }
            Action::PairStraight3(c) => {
                straight_to_string(StraightType::Pair, c, 3, Carry::None, Vec::new())
            }
            Action::PairStraight4(c) => {
                straight_to_string(StraightType::Pair, c, 4, Carry::None, Vec::new())
            }
            Action::PairStraight5(c) => {
                straight_to_string(StraightType::Pair, c, 5, Carry::None, Vec::new())
            }
            Action::PairStraight6(c) => {
                straight_to_string(StraightType::Pair, c, 6, Carry::None, Vec::new())
            }
            Action::PairStraight7(c) => {
                straight_to_string(StraightType::Pair, c, 7, Carry::None, Vec::new())
            }
            Action::PairStraight8(c) => {
                straight_to_string(StraightType::Pair, c, 8, Carry::None, Vec::new())
            }
            Action::PairStraight9(c) => {
                straight_to_string(StraightType::Pair, c, 9, Carry::None, Vec::new())
            }
            Action::PairStraight10(c) => {
                straight_to_string(StraightType::Pair, c, 10, Carry::None, Vec::new())
            }
            Action::Triple(c) => {
                straight_to_string(StraightType::Triple, c, 1, Carry::None, Vec::new())
            }
            Action::TripleSingle(c1, c2) => {
                straight_to_string(StraightType::Triple, c1, 1, Carry::Single, vec![c2])
            }
            Action::TriplePair(c1, c2) => {
                straight_to_string(StraightType::Triple, c1, 1, Carry::Pair, vec![c2])
            }
            Action::TripleStraight2(c) => {
                straight_to_string(StraightType::Triple, c, 2, Carry::None, Vec::new())
            }
            Action::TripleStraight2Single(c1, c2, c3) => {
                straight_to_string(StraightType::Triple, c1, 2, Carry::Single, vec![c2, c3])
            }
            Action::TripleStraight2Pair(c1, c2, c3) => {
                straight_to_string(StraightType::Triple, c1, 2, Carry::Pair, vec![c2, c3])
            }
            Action::TripleStraight3(c) => {
                straight_to_string(StraightType::Triple, c, 3, Carry::None, Vec::new())
            }
            Action::TripleStraight3Single(c1, c2, c3, c4) => {
                straight_to_string(StraightType::Triple, c1, 3, Carry::Single, vec![c2, c3, c4])
            }
            Action::TripleStraight3Pair(c1, c2, c3, c4) => {
                straight_to_string(StraightType::Triple, c1, 3, Carry::Pair, vec![c2, c3, c4])
            }
            Action::TripleStraight4(c) => {
                straight_to_string(StraightType::Triple, c, 4, Carry::None, Vec::new())
            }
            Action::TripleStraight4Single(c1, c2, c3, c4, c5) => straight_to_string(
                StraightType::Triple,
                c1,
                4,
                Carry::Single,
                vec![c2, c3, c4, c5],
            ),
            Action::TripleStraight4Pair(c1, c2, c3, c4, c5) => straight_to_string(
                StraightType::Triple,
                c1,
                4,
                Carry::Pair,
                vec![c2, c3, c4, c5],
            ),
            Action::TripleStraight5(c) => {
                straight_to_string(StraightType::Triple, c, 5, Carry::None, Vec::new())
            }
            Action::TripleStraight5Single(c1, c2, c3, c4, c5, c6) => straight_to_string(
                StraightType::Triple,
                c1,
                5,
                Carry::Single,
                vec![c2, c3, c4, c5, c6],
            ),
            Action::TripleStraight6(c) => {
                straight_to_string(StraightType::Triple, c, 6, Carry::None, Vec::new())
            }
            Action::BombSingle(c1, c2, c3) => format!("{}{}{}{}{}{}", c1, c1, c1, c1, c2, c3),
            Action::BombPair(c1, c2, c3) => {
                format!("{}{}{}{}{}{}{}{}", c1, c1, c1, c1, c2, c2, c3, c3)
            }
            Action::Bomb(c) => format!("{}{}{}{}", c, c, c, c),
            Action::Rocket => "BR".to_string(),
        };
        write!(f, "{}", s)
    }
}

fn straight_to_string(
    st: StraightType,
    card: &Card,
    length: u8,
    carry: Carry,
    carry_cards: Vec<&Card>,
) -> String {
    let mut s = String::new();
    let mut card = *card;
    for _ in 0..length {
        let count = match st {
            StraightType::Single => 1_u8,
            StraightType::Pair => 2,
            StraightType::Triple => 3,
        };
        for _ in 0..count {
            s.push_str(&card.to_string());
        }
        card = card.plus().unwrap();
    }

    match carry {
        Carry::None => {}
        Carry::Single => {
            for c in carry_cards {
                s.push_str(&c.to_string());
            }
        }
        Carry::Pair => {
            for c in carry_cards {
                s.push_str(&c.to_string());
                s.push_str(&c.to_string());
            }
        }
    }
    s
}
