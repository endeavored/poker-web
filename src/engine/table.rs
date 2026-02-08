use std::collections::HashMap;

use crate::engine::card;
use crate::engine::player;

#[derive(Hash)]
pub struct TablePlayer {
    active: bool,
    hand: (card::Card, card::Card),
    stack: u32,
    position: u8,
}

pub struct Table {
    dealer_location: u8,
    player_list: HashMap<player::PlayerId, TablePlayer>,
    pot: u32,
    max_table_size: u8,
}
