use crate::engine::card;

#[derive(Hash)]
pub struct HoldEmHand(card::Card, card::Card);
