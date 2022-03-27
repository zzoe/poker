use std::fmt::{Display, Formatter};
use std::str::FromStr;

use indextree::{Arena, NodeId};

use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Card {
    Three = 1,
    Four = 1 << 1,
    Five = 1 << 2,
    Six = 1 << 3,
    Seven = 1 << 4,
    Eight = 1 << 5,
    Nine = 1 << 6,
    Ten = 1 << 7,
    Jack = 1 << 8,
    Queen = 1 << 9,
    King = 1 << 10,
    Ace = 1 << 11,
    Two = 1 << 12,
    BlackJoker = 1 << 13,
    RedJoker = 1 << 14,
}

enum StraightType {
    Single,
    Pair,
    Triple,
}

enum Carry {
    None,
    Single,
    Pair,
}

impl Card {
    fn from_char(c: char) -> Result<Card, Error> {
        match c {
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            '0' => Ok(Card::Ten),
            'j' | 'J' => Ok(Card::Jack),
            'q' | 'Q' => Ok(Card::Queen),
            'k' | 'K' => Ok(Card::King),
            '1' | 'a' | 'A' => Ok(Card::Ace),
            '2' => Ok(Card::Two),
            'b' | 'B' => Ok(Card::BlackJoker),
            'r' | 'R' => Ok(Card::RedJoker),
            _ => Err(Error::InvalidCardValue(c.to_string())),
        }
    }

    fn from_u16(n: u16) -> Result<Card, Error> {
        match n {
            1 => Ok(Card::Three),
            0b10 => Ok(Card::Four),
            0b100 => Ok(Card::Five),
            0b1000 => Ok(Card::Six),
            0b10000 => Ok(Card::Seven),
            0b100000 => Ok(Card::Eight),
            0b1000000 => Ok(Card::Nine),
            0b10000000 => Ok(Card::Ten),
            0b100000000 => Ok(Card::Jack),
            0b1000000000 => Ok(Card::Queen),
            0b10000000000 => Ok(Card::King),
            0b100000000000 => Ok(Card::Ace),
            0b1000000000000 => Ok(Card::Two),
            0b10000000000000 => Ok(Card::BlackJoker),
            0b100000000000000 => Ok(Card::RedJoker),
            _ => Err(Error::InvalidCardValue(n.to_string())),
        }
    }

    fn plus(&self) -> Option<Card> {
        if Card::RedJoker == *self {
            None
        } else {
            Card::from_u16((*self as u16) << 1).ok()
        }
    }

    fn minus(&self) -> Option<Card> {
        if Card::Three == *self {
            None
        } else {
            Card::from_u16((*self as u16) >> 1).ok()
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct Hand(u64);

impl Hand {
    fn new(cards: &str) -> Result<Hand, Error> {
        if cards.is_empty() {
            return Err(Error::Empty);
        }

        let mut hand = Hand(0);
        hand.draw_str(cards);
        Ok(hand)
    }

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

    /// 摸牌
    fn draw_str(&mut self, cards: &str) {
        for c in cards.chars() {
            match Card::from_char(c) {
                Ok(card) => self.draw_card(card),
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn play(&self, action: &Action) -> Vec<(Action, Hand)> {
        let mut not_bomb = true;
        let mut actions = match action {
            Action::None => self.play_any(),
            Action::Single(card) => self.play_single(Some(card)),
            Action::Straight5(c) => self.play_straight(StraightType::Single, Some(c), 5),
            Action::Straight6(c) => self.play_straight(StraightType::Single, Some(c), 6),
            Action::Straight7(c) => self.play_straight(StraightType::Single, Some(c), 7),
            Action::Straight8(c) => self.play_straight(StraightType::Single, Some(c), 8),
            Action::Straight9(c) => self.play_straight(StraightType::Single, Some(c), 9),
            Action::Straight10(c) => self.play_straight(StraightType::Single, Some(c), 10),
            Action::Straight11(c) => self.play_straight(StraightType::Single, Some(c), 11),
            Action::Straight12 => Vec::new(),
            Action::Pair(c) => self.play_pair(Some(c)),
            Action::PairStraight3(c) => self.play_straight(StraightType::Pair, Some(c), 3),
            Action::PairStraight4(c) => self.play_straight(StraightType::Pair, Some(c), 4),
            Action::PairStraight5(c) => self.play_straight(StraightType::Pair, Some(c), 5),
            Action::PairStraight6(c) => self.play_straight(StraightType::Pair, Some(c), 6),
            Action::PairStraight7(c) => self.play_straight(StraightType::Pair, Some(c), 7),
            Action::PairStraight8(c) => self.play_straight(StraightType::Pair, Some(c), 8),
            Action::PairStraight9(c) => self.play_straight(StraightType::Pair, Some(c), 9),
            Action::PairStraight10(c) => self.play_straight(StraightType::Pair, Some(c), 10),
            Action::Triple(c) => self.play_triple(Some(c), Carry::None),
            Action::TripleSingle(c, _) => self.play_triple(Some(c), Carry::Single),
            Action::TriplePair(c, _) => self.play_triple(Some(c), Carry::Pair),
            Action::TripleStraight2(c) => self.play_straight(StraightType::Triple, Some(c), 2),
            Action::TripleStraight2Single(c, _, _) => {
                self.play_triple_straight(Some(c), Carry::Single, 2)
            }
            Action::TripleStraight2Pair(c, _, _) => {
                self.play_triple_straight(Some(c), Carry::Pair, 2)
            }
            Action::TripleStraight3(c) => self.play_straight(StraightType::Triple, Some(c), 3),
            Action::TripleStraight3Single(c, ..) => {
                self.play_triple_straight(Some(c), Carry::Single, 3)
            }
            Action::TripleStraight3Pair(c, _, _, _) => {
                self.play_triple_straight(Some(c), Carry::Pair, 3)
            }
            Action::TripleStraight4(c) => self.play_straight(StraightType::Triple, Some(c), 4),
            Action::TripleStraight4Single(c, _, _, _, _) => {
                self.play_triple_straight(Some(c), Carry::Single, 4)
            }
            Action::TripleStraight4Pair(c, _, _, _, _) => {
                self.play_triple_straight(Some(c), Carry::Pair, 4)
            }
            Action::TripleStraight5(c) => self.play_straight(StraightType::Triple, Some(c), 5),
            Action::TripleStraight5Single(c, _, _, _, _, _) => {
                self.play_triple_straight(Some(c), Carry::Single, 5)
            }
            Action::TripleStraight6(c) => self.play_straight(StraightType::Triple, Some(c), 6),
            Action::BombSingle(c, _, _) => self.play_bomb_carry(Some(c), Carry::Single),
            Action::BombPair(c, _, _) => self.play_bomb_carry(Some(c), Carry::Pair),
            Action::Bomb(c) => {
                not_bomb = false;
                self.play_bomb(Some(c))
            }
            Action::Rocket => {
                not_bomb = false;
                Vec::new()
            }
        };

        if not_bomb {
            actions.extend_from_slice(&self.play_bomb(None));
        }

        if actions.is_empty() {
            actions.push((Action::None, *self));
        }
        actions
    }

    fn play_any(&self) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        //20张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Single, 5));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 10));
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Pair, 4));
        //18张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Triple, None, 6));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 9));
        //16张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Single, 4));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 8));
        //15张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Pair, 3));
        actions.extend_from_slice(&self.play_straight(StraightType::Triple, None, 5));
        //14张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 7));
        //12张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Single, 3));
        actions.extend_from_slice(&self.play_straight(StraightType::Triple, None, 4));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 6));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 12));
        //11张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 11));
        //10张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Pair, 2));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 5));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 10));
        //9张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Triple, None, 3));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 9));
        //8张牌
        actions.extend_from_slice(&self.play_triple_straight(None, Carry::Single, 2));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 4));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 8));
        //7张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 7));
        //6张牌
        actions.extend_from_slice(&self.play_straight(StraightType::Triple, None, 2));
        actions.extend_from_slice(&self.play_straight(StraightType::Pair, None, 3));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 6));
        //5张牌
        actions.extend_from_slice(&self.play_triple(None, Carry::Pair));
        actions.extend_from_slice(&self.play_straight(StraightType::Single, None, 5));
        //3张牌
        actions.extend_from_slice(&self.play_triple(None, Carry::None));
        //4张牌
        actions.extend_from_slice(&self.play_triple(None, Carry::Single));
        //2张牌
        actions.extend_from_slice(&self.play_pair(None));
        //1张牌
        actions.extend_from_slice(&self.play_single(None));
        //8张牌
        actions.extend_from_slice(&self.play_bomb_carry(None, Carry::Pair));
        //6张牌
        actions.extend_from_slice(&self.play_bomb_carry(None, Carry::Single));
        actions
    }

    fn play_single(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        loop {
            if let Some(c) = card {
                let mut hand = *self;
                if hand.play_card(&c) {
                    actions.push((Action::Single(c), hand));
                }
                card = c.plus();
            } else {
                return actions;
            }
        }
    }

    fn play_straight(
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
            if let Some(c) = card {
                straight |= c as u16;
                card = c.plus();
            } else {
                return actions;
            }
        }

        while straight < Card::Two as u16 {
            let action = match st {
                StraightType::Single => self.strait_single(straight, straight_start, length),
                StraightType::Pair => self.straight_pair(straight as u64, straight_start, length),
                StraightType::Triple => {
                    self.straight_triple(straight as u64, straight_start, length)
                }
            };
            actions.extend_from_slice(&*action);
            straight_start = straight_start.plus().unwrap();
            straight <<= 1;
        }

        actions
    }

    fn strait_single(
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
                hand.play_card(&card);
                card = card.plus().unwrap();
            }
            match length {
                5 => actions.push((Action::Straight5(straight_start), hand)),
                6 => actions.push((Action::Straight6(straight_start), hand)),
                7 => actions.push((Action::Straight7(straight_start), hand)),
                8 => actions.push((Action::Straight8(straight_start), hand)),
                9 => actions.push((Action::Straight9(straight_start), hand)),
                10 => actions.push((Action::Straight10(straight_start), hand)),
                11 => actions.push((Action::Straight11(straight_start), hand)),
                12 => actions.push((Action::Straight12, hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn straight_pair(
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
                hand.play_card(&card);
                hand.play_card(&card);
                card = card.plus().unwrap();
            }
            match length {
                3 => actions.push((Action::PairStraight3(straight_start), hand)),
                4 => actions.push((Action::PairStraight4(straight_start), hand)),
                5 => actions.push((Action::PairStraight5(straight_start), hand)),
                6 => actions.push((Action::PairStraight6(straight_start), hand)),
                7 => actions.push((Action::PairStraight7(straight_start), hand)),
                8 => actions.push((Action::PairStraight8(straight_start), hand)),
                9 => actions.push((Action::PairStraight9(straight_start), hand)),
                10 => actions.push((Action::PairStraight10(straight_start), hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn straight_triple(
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
                hand.play_card(&card);
                hand.play_card(&card);
                hand.play_card(&card);
                card = card.plus().unwrap();
            }
            match length {
                2 => actions.push((Action::TripleStraight2(straight_start), hand)),
                3 => actions.push((Action::TripleStraight3(straight_start), hand)),
                4 => actions.push((Action::TripleStraight4(straight_start), hand)),
                5 => actions.push((Action::TripleStraight5(straight_start), hand)),
                6 => actions.push((Action::TripleStraight6(straight_start), hand)),
                _ => unreachable!(),
            }
        }
        actions
    }

    fn play_pair(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        loop {
            if let Some(c) = card {
                let pair = c as u64 | (c as u64) << 16;
                let mut hand = *self;
                if hand.0 & pair == pair {
                    hand.play_card(&c);
                    hand.play_card(&c);
                    actions.push((Action::Pair(c), hand));
                }

                if c == Card::Two {
                    return actions;
                }
                card = c.plus();
            } else {
                return actions;
            }
        }
    }

    fn play_triple(&self, card: Option<&Card>, carry: Carry) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        loop {
            if let Some(c) = card {
                let triple = c as u64 | (c as u64) << 16 | (c as u64) << 32;
                let mut hand = *self;
                if hand.0 & triple == triple {
                    hand.play_card(&c);
                    hand.play_card(&c);
                    hand.play_card(&c);
                    match carry {
                        Carry::None => {
                            actions.push((Action::Triple(c), hand));
                        }
                        Carry::Single => {
                            for (carry, hand) in hand.play_triple_single(&c) {
                                actions.push((Action::TripleSingle(c, carry), hand))
                            }
                        }
                        Carry::Pair => {
                            for (carry, hand) in hand.play_triple_pair(&c) {
                                actions.push((Action::TriplePair(c, carry), hand))
                            }
                        }
                    }
                }

                if c == Card::Two {
                    return actions;
                }
                card = c.plus();
            } else {
                return actions;
            }
        }
    }

    fn play_triple_single(&self, card: &Card) -> Vec<(Card, Hand)> {
        let mut actions = Vec::new();
        for (a, h) in self.play_single(None) {
            if let Action::Single(c) = a {
                if c != *card {
                    actions.push((c, h));
                }
            }
        }

        actions
    }

    fn play_triple_pair(&self, card: &Card) -> Vec<(Card, Hand)> {
        let mut actions = Vec::new();
        for (a, h) in self.play_pair(None) {
            if let Action::Pair(c) = a {
                if c != *card {
                    actions.push((c, h));
                }
            }
        }

        actions
    }

    fn play_triple_straight(
        &self,
        card: Option<&Card>,
        carry: Carry,
        length: u8,
    ) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let hand = *self;

        for (action, hand) in hand.play_straight(StraightType::Triple, card, length) {
            let straight_start = match action {
                Action::TripleStraight2(c) => c,
                Action::TripleStraight3(c) => c,
                Action::TripleStraight4(c) => c,
                Action::TripleStraight5(c) => c,
                Action::TripleStraight6(c) => c,
                _ => unreachable!(),
            };
            match carry {
                Carry::Single => {
                    let mut carry_actions = Vec::new();
                    Self::carry_single(&mut carry_actions, Vec::new(), hand, length);
                    for (a, h) in carry_actions {
                        match length {
                            2 => actions.push((
                                Action::TripleStraight2Single(straight_start, a[0], a[1]),
                                h,
                            )),
                            3 => actions.push((
                                Action::TripleStraight3Single(straight_start, a[0], a[1], a[2]),
                                h,
                            )),
                            4 => actions.push((
                                Action::TripleStraight4Single(
                                    straight_start,
                                    a[0],
                                    a[1],
                                    a[2],
                                    a[3],
                                ),
                                h,
                            )),
                            5 => actions.push((
                                Action::TripleStraight5Single(
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
                            2 => actions
                                .push((Action::TripleStraight2Pair(straight_start, a[0], a[1]), h)),
                            3 => actions.push((
                                Action::TripleStraight3Pair(straight_start, a[0], a[1], a[2]),
                                h,
                            )),
                            4 => actions.push((
                                Action::TripleStraight4Pair(straight_start, a[0], a[1], a[2], a[3]),
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
        lenght: u8,
    ) {
        let mut min_card = None;
        if let Some(card) = cards.last() {
            min_card = card.minus();
        }
        hand.play_single(min_card.as_ref())
            .into_iter()
            .for_each(|(a, h)| {
                if let Action::Single(c) = a {
                    let mut cards = cards.clone();
                    cards.push(c);
                    if lenght > 1 {
                        Self::carry_single(actions, cards, h, lenght - 1);
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
        hand.play_pair(min_card.as_ref())
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

    fn play_bomb_carry(&self, card: Option<&Card>, carry: Carry) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        for (action, hand) in self.play_bomb(card) {
            if let Action::Bomb(c) = action {
                let mut carry_actions = Vec::new();
                match carry {
                    Carry::Single => {
                        Self::carry_single(&mut carry_actions, Vec::new(), hand, 2);
                        for (a, h) in carry_actions {
                            actions.push((Action::BombSingle(c, a[0], a[1]), h));
                        }
                    }
                    Carry::Pair => {
                        Self::carry_pair(&mut carry_actions, Vec::new(), hand, 2);
                        for (a, h) in carry_actions {
                            actions.push((Action::BombPair(c, a[0], a[1]), h));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        actions
    }

    fn play_bomb(&self, card: Option<&Card>) -> Vec<(Action, Hand)> {
        let mut actions = Vec::new();
        let mut card = Self::plus(card);

        loop {
            if let Some(c) = card {
                let cv = c as u64;
                let bomb = cv | cv << 16 | cv << 32 | cv << 48;
                let mut hand = *self;

                if hand.0 & bomb == bomb {
                    hand.0 &= !bomb;
                    actions.push((Action::Bomb(c), hand));
                }
                card = c.plus();
            } else {
                break;
            }
        }

        let rocket = Card::BlackJoker as u64 | Card::RedJoker as u64;
        if self.0 & rocket == rocket {
            let mut hand = *self;
            hand.0 &= !rocket;
            actions.push((Action::Rocket, hand));
        }

        actions
    }

    fn play_card(&mut self, card: &Card) -> bool {
        let mut card = (*card as u64) << 48;
        for _ in 0..4 {
            if self.0 & card == card {
                self.0 &= !card;
                return true;
            }
            card >>= 16;
        }
        false
    }

    fn play_str(&mut self, cards: &str) {
        for c in cards.chars() {
            match Card::from_char(c) {
                Ok(card) => {
                    self.play_card(&card);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn plus(card: Option<&Card>) -> Option<Card> {
        match card {
            Some(c) => c.plus(),
            None => Some(Card::Three),
        }
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

/// 牌组
#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
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
}

pub struct Game {
    arena: Arena<State>,
    root: NodeId,
}

impl Game {
    pub fn new(player_str: Vec<String>, turn: u8) -> Result<Self, Error> {
        let mut arena = Arena::new();
        let root = arena.new_node(State::new(player_str, turn)?);
        Ok(Game { arena, root })
    }

    pub fn pass(&self) -> bool {
        self.arena.get(self.root).unwrap().get().pass
    }

    pub fn play(&mut self) {
        self.play_next(Some(self.root));
    }

    fn play_next(&mut self, node_id: Option<NodeId>) {
        if let Some(n) = node_id {
            let state = self.arena.get(n).unwrap().get();
            let next_node_id = if !state.player.is_empty() {
                // node_id: 当前节点有player
                self.expand(n)
            } else if state.turn == 0 {
                // node_id: 当前节点没有player, turn: 0
                self.expand_player1(n)
            } else {
                // node_id: 当前节点没有player, turn != 0
                self.expand_other_player(n)
            };
            self.play_next(next_node_id);
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
    pub fn expand(&mut self, node_id: NodeId) -> Option<NodeId> {
        let mut state = self.arena.get(node_id)?.get().clone();
        let mut next_node_id = None;
        let turn = state.turn as usize;
        let hand = match state.player.get(turn) {
            Some(h) => {
                if h.0 == 0 {
                    eprintln!("手牌为空？ {}", state);
                    return None;
                }
                h
            }
            None => {
                eprintln!("手牌为空？ {}", state);
                return None;
            }
        };

        for (action, hand) in hand.play(&state.action) {
            state.player[turn] = hand;
            let pass = hand.0 == 0;
            let child = self.arena.new_node(State {
                action,
                player: state.player.clone(),
                turn: (state.turn + 1) % state.player.len() as u8,
                pass: pass && turn == 0,
            });
            node_id.append(child, &mut self.arena);
            // println!(
            //     "parent: {}, child:{}, node:{}",
            //     node_id,
            //     child,
            //     self.arena.get(child).unwrap().get()
            // );

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
        // println!(
        //     "node_id: {}, delete subtree of: {}, next: {:?}",
        //     node_id, last_node_id, next_node_id
        // );
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
                // println!("passed_node_id: {}, delete sibling: {}", passed_node_id, n);
                n.remove(&mut self.arena)
            });
        passed_node_id
            .following_siblings(&self.arena)
            .skip(1)
            .collect::<Vec<NodeId>>()
            .iter()
            .for_each(|n| {
                // println!("passed_node_id: {}, delete sibling: {}", passed_node_id, n);
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
    fn test_draw() {
        let hand = Hand::new("345343BR").unwrap();
        println!("{}", hand);
        assert_eq!(
            hand.0,
            1 | 1 << 1 | 1 << 2 | 1 << 16 | 1 << 17 | 1 << 32 | 1 << 13 | 1 << 14
        );
    }

    #[test]
    fn test_play() {
        let mut hand = Hand::default();
        hand.draw_str("343353BR");
        hand.play_str("4R");
        println!("{}", hand);
        assert_eq!(hand.0, 1 << 48 | 1 << 32 | 1 << 16 | 1 << 2 | 1 | 1 << 13);
    }

    #[test]
    fn test_state_play() {
        let mut game = Game::new(vec!["123".to_string(), "234".to_string()], 0).unwrap();
        game.play();
        game.print();
        assert!(game.pass());

        let mut game = Game::new(vec!["34".to_string(), "5".to_string()], 0).unwrap();
        game.play();
        game.print();
        assert!(!game.pass());
    }
}
