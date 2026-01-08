use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        Self::translate(rng.gen_range(1..=4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value. Expected 1-4, got {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        Self::translate(rng.gen_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value. Expected 1-13, got {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}