use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::engine::card;
use crate::engine::random;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<card::Card>,
}

impl Default for Deck {
    fn default() -> Deck {
        let product = card::Suit::iter().cartesian_product(card::Rank::iter());
        let iter = product.map(|(s, r)| card::Card { suit: s, rank: r });
        Deck {
            cards: iter.collect::<Vec<card::Card>>(),
        }
    }
}

impl Deck {
    pub fn draw_card(&mut self) -> Option<card::Card> {
        if self.cards.is_empty() {
            return None;
        }
        let ind = random::RandomApi::rand_int(0, self.cards.len() as u32) as usize;
        let chosen: card::Card = self.cards.remove(ind);
        // TODO: Add provably fair return
        Some(chosen)
    }
}
