use crate::engine::card;

#[derive(Hash, Eq, PartialEq)]
pub struct HoldEmHand(card::Card, card::Card);
