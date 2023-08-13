use crate::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SuitCard {
    /// ♠黑桃
    Spades(Card),
    /// ♥红心
    Hearts(Card),
    /// ♣梅花
    Clubs(Card),
    /// ♦方块, 大小王暂时也在这里
    Diamonds(Card),
}

impl SuitCard{
    pub fn new(card: Card, suit: u8) -> Self{
        match suit {
            3 => SuitCard::Spades(card),
            2 => SuitCard::Hearts(card),
            1 => SuitCard::Clubs(card),
            0 => SuitCard::Diamonds(card),
            _ => unreachable!(),
        }
    }
}

impl From<SuitCard> for u64{
    fn from(suit_card: SuitCard) -> Self {
        match suit_card {
            SuitCard::Spades(card) => {
                // log::debug!("{:064b}", card as u64);
                (card as u64) << 48
            },
            SuitCard::Hearts(card) => (card as u64) << 32,
            SuitCard::Clubs(card) => (card as u64) << 16,
            SuitCard::Diamonds(card) => card as u64,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
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

impl Card {
    pub(crate) fn from_char(c: char) -> Result<Card, Error> {
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

    pub fn from_u16(n: u16) -> Result<Card, Error> {
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

    pub fn plus(&self) -> Option<Card> {
        if Card::RedJoker == *self {
            None
        } else {
            Card::from_u16((*self as u16) << 1).ok()
        }
    }

    pub(crate) fn minus(&self) -> Option<Card> {
        if Card::Three == *self {
            None
        } else {
            Card::from_u16((*self as u16) >> 1).ok()
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => '0',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
            Card::Two => '2',
            Card::BlackJoker => 'B',
            Card::RedJoker => 'R',
        };
        write!(f, "{}", s)
    }
}