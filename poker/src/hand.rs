use std::fmt::Display;

use crate::action::Action;
use crate::card::{Card, SuitCard};
use crate::game::{Carry, StraightType};

/// 用u64表示一副牌，每16位代表一个花色，分别是桃仙梅方；用后15位分别表示大王、小王、2、A、K、Q、J、10、9、8、7、6、5、4、3
pub const DECK_OF_CARDS: Hand =
    Hand(0b0001111111111111000111111111111100011111111111110111111111111111);

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Hand(u64);

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut hand = Hand(0);
        for c in value.chars() {
            match Card::from_char(c) {
                Ok(card) => hand.draw_card(card),
                Err(e) => log::error!("{}", e),
            }
        }
        hand
    }
}

impl Hand {
    pub fn value(&self) -> u64 {
        self.0
    }

    /// 手牌张数
    pub fn size(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn contains(&self, suit_card: SuitCard) -> bool {
        let c = u64::from(suit_card);
        self.0 & c == c
    }

    pub fn insert_suit_card(&mut self, suit_card: SuitCard) {
        self.0 |= u64::from(suit_card);
    }

    pub fn remove_suit_card(&mut self, suit_card: SuitCard) {
        self.0 &= !u64::from(suit_card)
    }

    pub fn insert_hand(&mut self, hand: Hand) {
        self.0 |= hand.0
    }

    pub fn remove_hand(&mut self, hand: Hand) {
        self.0 &= !hand.0
    }
}

impl Hand {
    /// 无视花色的手牌整理，高位的1跟低位的0互换
    pub(crate) fn arrange(&self) -> Hand {
        let mut segments = [
            self.0 & 0xFFFF,
            (self.0 >> 16) & 0xFFFF,
            (self.0 >> 32) & 0xFFFF,
            (self.0 >> 48) & 0xFFFF,
        ];

        for i in (1..4).rev() {
            for j in 0..i {
                if segments[i] == 0 {
                    break;
                }

                //相同的位是0，不同的位是1
                let different = segments[i] ^ segments[j];
                //相同的位是1，不同的位是0
                let identical = !different;
                // 相同的位保持不变，不同的位设为0
                segments[i] &= identical;
                // 相同的位保持不变，不同的位设为1
                segments[j] |= different;
            }
        }

        Hand(segments.iter().rev().fold(0, |v, s| s | (v << 16)))
    }

    /// 抓牌，不考虑花色，优先放在低位
    fn draw_card(&mut self, card: Card) {
        let mut card = card as u64;
        for _ in 0..4 {
            if self.0 & card == 0 {
                self.0 |= card;
                break;
            }
            card <<= 16;
        }
    }

    /// 打出一张牌
    pub fn play_card(&mut self, card: Card) -> Option<SuitCard> {
        for i in (0..4).rev() {
            let suit_card = SuitCard::new(card, i);
            let c = u64::from(suit_card);
            if self.0 & c == c {
                self.0 &= !c;
                return Some(suit_card);
            }
        }
        None
    }

    /// “管上”指定牌组的所有方案
    pub(crate) fn follow(&self, action: &Action) -> Vec<(Action, Hand)> {
        let mut own_round = false;
        let mut not_bomb = true;
        let mut actions = match action {
            Action::None => {
                own_round = true;
                self.follow_any()
            }
            Action::Single(card) => self.follow_single(Some(card)),
            Action::Sequence5(c) => self.follow_straight(StraightType::Single, Some(c), 5),
            Action::Sequence6(c) => self.follow_straight(StraightType::Single, Some(c), 6),
            Action::Sequence7(c) => self.follow_straight(StraightType::Single, Some(c), 7),
            Action::Sequence8(c) => self.follow_straight(StraightType::Single, Some(c), 8),
            Action::Sequence9(c) => self.follow_straight(StraightType::Single, Some(c), 9),
            Action::Sequence10(c) => self.follow_straight(StraightType::Single, Some(c), 10),
            Action::Sequence11(c) => self.follow_straight(StraightType::Single, Some(c), 11),
            Action::Sequence12 => Vec::new(),
            Action::Pair(c) => self.follow_pair(Some(c)),
            Action::PairSequence3(c) => self.follow_straight(StraightType::Pair, Some(c), 3),
            Action::PairSequence4(c) => self.follow_straight(StraightType::Pair, Some(c), 4),
            Action::PairSequence5(c) => self.follow_straight(StraightType::Pair, Some(c), 5),
            Action::PairSequence6(c) => self.follow_straight(StraightType::Pair, Some(c), 6),
            Action::PairSequence7(c) => self.follow_straight(StraightType::Pair, Some(c), 7),
            Action::PairSequence8(c) => self.follow_straight(StraightType::Pair, Some(c), 8),
            Action::PairSequence9(c) => self.follow_straight(StraightType::Pair, Some(c), 9),
            Action::PairSequence10(c) => self.follow_straight(StraightType::Pair, Some(c), 10),
            Action::Triplet(c) => self.follow_triple(Some(c), Carry::None),
            Action::TripletSingle(c, _) => self.follow_triple(Some(c), Carry::Single),
            Action::TripletPair(c, _) => self.follow_triple(Some(c), Carry::Pair),
            Action::TripletSequence2(c) => self.follow_straight(StraightType::Triple, Some(c), 2),
            Action::TripletSequence2Single(c, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Single, 2)
            }
            Action::TripletSequence2Pair(c, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Pair, 2)
            }
            Action::TripletSequence3(c) => self.follow_straight(StraightType::Triple, Some(c), 3),
            Action::TripletSequence3Single(c, ..) => {
                self.follow_triple_straight(Some(c), Carry::Single, 3)
            }
            Action::TripletSequence3Pair(c, _, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Pair, 3)
            }
            Action::TripletSequence4(c) => self.follow_straight(StraightType::Triple, Some(c), 4),
            Action::TripletSequence4Single(c, _, _, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Single, 4)
            }
            Action::TripletSequence4Pair(c, _, _, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Pair, 4)
            }
            Action::TripletSequence5(c) => self.follow_straight(StraightType::Triple, Some(c), 5),
            Action::TripletSequence5Single(c, _, _, _, _, _) => {
                self.follow_triple_straight(Some(c), Carry::Single, 5)
            }
            Action::TripletSequence6(c) => self.follow_straight(StraightType::Triple, Some(c), 6),
            Action::QuadSingle(c, _, _) => self.follow_bomb_carry(Some(c), Carry::Single),
            Action::QuadPair(c, _, _) => self.follow_bomb_carry(Some(c), Carry::Pair),
            Action::Bomb(c) => {
                not_bomb = false;
                self.follow_bomb(Some(c))
            }
            Action::Rocket => {
                not_bomb = false;
                Vec::new()
            }
        };

        if not_bomb {
            actions.extend_from_slice(&self.follow_bomb(None));
        }

        if !own_round {
            actions.push((Action::None, *self));
        }
        actions
    }

    fn follow_any(&self) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        //20张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Single, 5));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 10));
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Pair, 4));
        //18张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Triple, None, 6));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 9));
        //16张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Single, 4));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 8));
        //15张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Pair, 3));
        actions.extend_from_slice(&self.follow_straight(StraightType::Triple, None, 5));
        //14张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 7));
        //12张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Single, 3));
        actions.extend_from_slice(&self.follow_straight(StraightType::Triple, None, 4));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 6));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 12));
        //11张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 11));
        //10张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Pair, 2));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 5));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 10));
        //9张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Triple, None, 3));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 9));
        //8张牌
        actions.extend_from_slice(&self.follow_triple_straight(None, Carry::Single, 2));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 4));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 8));
        //7张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 7));
        //6张牌
        actions.extend_from_slice(&self.follow_straight(StraightType::Triple, None, 2));
        actions.extend_from_slice(&self.follow_straight(StraightType::Pair, None, 3));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 6));
        //5张牌
        actions.extend_from_slice(&self.follow_triple(None, Carry::Pair));
        actions.extend_from_slice(&self.follow_straight(StraightType::Single, None, 5));
        //3张牌
        actions.extend_from_slice(&self.follow_triple(None, Carry::None));
        //4张牌
        actions.extend_from_slice(&self.follow_triple(None, Carry::Single));
        //2张牌
        actions.extend_from_slice(&self.follow_pair(None));
        //1张牌
        actions.extend_from_slice(&self.follow_single(None));
        //8张牌
        actions.extend_from_slice(&self.follow_bomb_carry(None, Carry::Pair));
        //6张牌
        actions.extend_from_slice(&self.follow_bomb_carry(None, Carry::Single));
        actions
    }

    fn follow_single(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        loop {
            let Some(c) = card else {
                return actions;
            };

            let mut hand = *self;
            if hand.play_card(c).is_some() {
                actions.push((Action::Single(c), hand));
            }
            card = c.plus();
        }
    }

    fn follow_straight(
        &self,
        st: StraightType,
        card: Option<&Card>,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);
        let mut straight = 0_u16;
        let mut straight_start = card.unwrap();

        for _ in 0..length {
            let Some(c) = card else { return actions };
            straight |= c as u16;
            card = c.plus();
        }

        while straight < Card::Two as u16 {
            let action = match st {
                StraightType::Single => self.follow_strait_single(straight, straight_start, length),
                StraightType::Pair => {
                    self.follow_straight_pair(straight as u64, straight_start, length)
                }
                StraightType::Triple => {
                    self.follow_straight_triple(straight as u64, straight_start, length)
                }
            };
            actions.extend_from_slice(&action);
            straight_start = straight_start.plus().unwrap();
            straight <<= 1;
        }

        actions
    }

    fn follow_strait_single(
        &self,
        straight: u16,
        straight_start: Card,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut hand = *self;

        if hand.0 as u16 & straight == straight {
            let mut card = straight_start;
            for _ in 0..length {
                hand.play_card(card);
                card = card.plus().unwrap();
            }
            match length {
                5 => actions.push((Action::Sequence5(straight_start), hand)),
                6 => actions.push((Action::Sequence6(straight_start), hand)),
                7 => actions.push((Action::Sequence7(straight_start), hand)),
                8 => actions.push((Action::Sequence8(straight_start), hand)),
                9 => actions.push((Action::Sequence9(straight_start), hand)),
                10 => actions.push((Action::Sequence10(straight_start), hand)),
                11 => actions.push((Action::Sequence11(straight_start), hand)),
                12 => actions.push((Action::Sequence12, hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn follow_straight_pair(
        &self,
        straight: u64,
        straight_start: Card,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut hand = *self;
        let straight = straight | straight << 16;

        if hand.0 & straight == straight {
            let mut card = straight_start;
            for _ in 0..length {
                hand.play_card(card);
                hand.play_card(card);
                card = card.plus().unwrap();
            }
            match length {
                3 => actions.push((Action::PairSequence3(straight_start), hand)),
                4 => actions.push((Action::PairSequence4(straight_start), hand)),
                5 => actions.push((Action::PairSequence5(straight_start), hand)),
                6 => actions.push((Action::PairSequence6(straight_start), hand)),
                7 => actions.push((Action::PairSequence7(straight_start), hand)),
                8 => actions.push((Action::PairSequence8(straight_start), hand)),
                9 => actions.push((Action::PairSequence9(straight_start), hand)),
                10 => actions.push((Action::PairSequence10(straight_start), hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn follow_straight_triple(
        &self,
        straight: u64,
        straight_start: Card,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut hand = *self;
        let straight = straight | straight << 16 | straight << 32;

        if hand.0 & straight == straight {
            let mut card = straight_start;
            for _ in 0..length {
                hand.play_card(card);
                hand.play_card(card);
                hand.play_card(card);
                card = card.plus().unwrap();
            }
            match length {
                2 => actions.push((Action::TripletSequence2(straight_start), hand)),
                3 => actions.push((Action::TripletSequence3(straight_start), hand)),
                4 => actions.push((Action::TripletSequence4(straight_start), hand)),
                5 => actions.push((Action::TripletSequence5(straight_start), hand)),
                6 => actions.push((Action::TripletSequence6(straight_start), hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn follow_pair(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        while let Some(c) = card {
            let pair = c as u64 | (c as u64) << 16;
            let mut hand = *self;
            if hand.0 & pair == pair {
                hand.play_card(c);
                hand.play_card(c);
                actions.push((Action::Pair(c), hand));
            }

            if c == Card::Two {
                return actions;
            }
            card = c.plus();
        }

        actions
    }

    fn follow_triple(&self, card: Option<&Card>, carry: Carry) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        while let Some(c) = card {
            let triple = c as u64 | (c as u64) << 16 | (c as u64) << 32;
            let mut hand = *self;
            if hand.0 & triple == triple {
                hand.play_card(c);
                hand.play_card(c);
                hand.play_card(c);
                match carry {
                    Carry::None => {
                        actions.push((Action::Triplet(c), hand));
                    }
                    Carry::Single => {
                        for (carry, hand) in hand.follow_triple_single(&c) {
                            actions.push((Action::TripletSingle(c, carry), hand))
                        }
                    }
                    Carry::Pair => {
                        for (carry, hand) in hand.follow_triple_pair(&c) {
                            actions.push((Action::TripletPair(c, carry), hand))
                        }
                    }
                }
            }

            if c == Card::Two {
                return actions;
            }
            card = c.plus();
        }

        actions
    }

    fn follow_triple_single(&self, card: &Card) -> Vec<(Card, Hand)> {
        let mut actions = Vec::new();
        for (a, h) in self.follow_single(None) {
            if let Action::Single(c) = a {
                if c != *card {
                    actions.push((c, h));
                }
            }
        }

        actions
    }

    fn follow_triple_pair(&self, card: &Card) -> Vec<(Card, Hand)> {
        let mut actions = Vec::new();
        for (a, h) in self.follow_pair(None) {
            if let Action::Pair(c) = a {
                if c != *card {
                    actions.push((c, h));
                }
            }
        }

        actions
    }

    fn follow_triple_straight(
        &self,
        card: Option<&Card>,
        carry: Carry,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let hand = *self;

        for (action, hand) in hand.follow_straight(StraightType::Triple, card, length) {
            let straight_start = match action {
                Action::TripletSequence2(c) => c,
                Action::TripletSequence3(c) => c,
                Action::TripletSequence4(c) => c,
                Action::TripletSequence5(c) => c,
                Action::TripletSequence6(c) => c,
                _ => unreachable!(),
            };
            match carry {
                Carry::Single => {
                    let mut carry_actions = Vec::new();
                    Self::carry_single(&mut carry_actions, Vec::new(), hand, length);
                    for (a, h) in carry_actions {
                        match length {
                            2 => actions.push((
                                Action::TripletSequence2Single(straight_start, a[0], a[1]),
                                h,
                            )),
                            3 => actions.push((
                                Action::TripletSequence3Single(straight_start, a[0], a[1], a[2]),
                                h,
                            )),
                            4 => actions.push((
                                Action::TripletSequence4Single(
                                    straight_start,
                                    a[0],
                                    a[1],
                                    a[2],
                                    a[3],
                                ),
                                h,
                            )),
                            5 => actions.push((
                                Action::TripletSequence5Single(
                                    straight_start,
                                    a[0],
                                    a[1],
                                    a[2],
                                    a[3],
                                    a[4],
                                ),
                                h,
                            )),
                            _ => unreachable!(),
                        }
                    }
                }
                Carry::Pair => {
                    let mut carry_actions = Vec::new();
                    Self::carry_pair(&mut carry_actions, Vec::new(), hand, length);
                    for (a, h) in carry_actions {
                        match length {
                            2 => actions.push((
                                Action::TripletSequence2Pair(straight_start, a[0], a[1]),
                                h,
                            )),
                            3 => actions.push((
                                Action::TripletSequence3Pair(straight_start, a[0], a[1], a[2]),
                                h,
                            )),
                            4 => actions.push((
                                Action::TripletSequence4Pair(
                                    straight_start,
                                    a[0],
                                    a[1],
                                    a[2],
                                    a[3],
                                ),
                                h,
                            )),
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        actions
    }

    fn carry_single(
        actions: &mut Vec<(Vec<Card>, Hand)>,
        cards: Vec<Card>,
        hand: Hand,
        length: u8,
    ) {
        let mut min_card = None;
        if let Some(card) = cards.last() {
            min_card = card.minus();
        }
        hand.follow_single(min_card.as_ref())
            .into_iter()
            .for_each(|(a, h)| {
                if let Action::Single(c) = a {
                    let mut cards = cards.clone();
                    cards.push(c);
                    if length > 1 {
                        Self::carry_single(actions, cards, h, length - 1);
                    } else {
                        actions.push((cards, h));
                    }
                }
            });
    }

    fn carry_pair(actions: &mut Vec<(Vec<Card>, Hand)>, cards: Vec<Card>, hand: Hand, length: u8) {
        let mut min_card = None;
        if let Some(card) = cards.last() {
            min_card = card.minus();
        }
        hand.follow_pair(min_card.as_ref())
            .into_iter()
            .for_each(|(a, h)| {
                if let Action::Single(c) = a {
                    let mut cards = cards.clone();
                    cards.push(c);
                    if length > 1 {
                        Self::carry_pair(actions, cards, h, length - 1);
                    } else {
                        actions.push((cards, h));
                    }
                }
            });
    }

    fn follow_bomb_carry(&self, card: Option<&Card>, carry: Carry) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        for (action, hand) in self.follow_bomb(card) {
            if let Action::Bomb(c) = action {
                let mut carry_actions = Vec::new();
                match carry {
                    Carry::Single => {
                        Self::carry_single(&mut carry_actions, Vec::new(), hand, 2);
                        for (a, h) in carry_actions {
                            actions.push((Action::QuadSingle(c, a[0], a[1]), h));
                        }
                    }
                    Carry::Pair => {
                        Self::carry_pair(&mut carry_actions, Vec::new(), hand, 2);
                        for (a, h) in carry_actions {
                            actions.push((Action::QuadPair(c, a[0], a[1]), h));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        actions
    }

    fn follow_bomb(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        while let Some(c) = card {
            let cv = c as u64;
            let bomb = cv | cv << 16 | cv << 32 | cv << 48;
            let mut hand = *self;

            if hand.0 & bomb == bomb {
                hand.0 &= !bomb;
                actions.push((Action::Bomb(c), hand));
            }
            card = c.plus();
        }

        let rocket = Card::BlackJoker as u64 | Card::RedJoker as u64;
        if self.0 & rocket == rocket {
            let mut hand = *self;
            hand.0 &= !rocket;
            actions.push((Action::Rocket, hand));
        }

        actions
    }

    fn plus(card: Option<&Card>) -> Option<Card> {
        match card {
            Some(c) => c.plus(),
            None => Some(Card::Three),
        }
    }
}

impl Iterator for Hand {
    type Item = SuitCard;

    fn next(&mut self) -> Option<Self::Item> {
        let mut card = Card::RedJoker;
        loop {
            match self.play_card(card) {
                None => {
                    let next_card = card.minus()?;
                    card = next_card;
                }
                s => return s,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.size() as usize;
        (size, Some(size))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hand: ")?;
        for i in 0..16 {
            if (self.0 >> i) & 1 == 1 {
                write!(f, "{}", "34567890JQKA2BR".chars().nth(i).unwrap())?;
            }
            if (self.0 >> (i + 16)) & 1 == 1 {
                write!(f, "{}", "34567890JQKA2  ".chars().nth(i).unwrap())?;
            }
            if (self.0 >> (i + 32)) & 1 == 1 {
                write!(f, "{}", "34567890JQKA2  ".chars().nth(i).unwrap())?;
            }
            if (self.0 >> (i + 48)) & 1 == 1 {
                write!(f, "{}", "34567890JQKA2  ".chars().nth(i).unwrap())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw() {
        let hand = Hand::from("345343BR");
        log::debug!("{}", hand);
        assert_eq!(
            hand.0,
            1 | 1 << 1 | 1 << 2 | 1 << 16 | 1 << 17 | 1 << 32 | 1 << 13 | 1 << 14
        );
    }

    #[test]
    fn test_play() {
        let mut hand = Hand::from("343353BR");
        hand.play_card(Card::Four);
        hand.play_card(Card::RedJoker);
        log::debug!("{}", hand);
        assert_eq!(hand.0, 1 << 48 | 1 << 32 | 1 << 16 | 1 << 2 | 1 | 1 << 13);
    }

    #[test]
    fn test_suit_card() {
        let cards_except_joker = (0..52)
            .map(|u| SuitCard::new(Card::from_u16(1 << (u / 4)).unwrap(), (3 - u % 4) as u8))
            .collect::<Vec<SuitCard>>();

        let mut hand = DECK_OF_CARDS;
        for suit_card in cards_except_joker {
            hand.remove_suit_card(suit_card);
        }

        assert_eq!(hand.0, Card::BlackJoker as u64 | Card::RedJoker as u64);
    }
}
