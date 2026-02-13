use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
