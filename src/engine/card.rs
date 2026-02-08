#[derive(Hash)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(Hash)]
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

#[derive(Hash)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}
