use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Hash, Eq, PartialEq)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(Debug, EnumIter, Hash, Eq, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
