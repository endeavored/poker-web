#[derive(Hash, Eq, PartialEq)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(Hash, Eq, PartialEq)]
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

#[derive(Hash, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}
