use std::collections::HashMap;

use crate::engine::hand;
use crate::engine::player;

#[derive(Hash, Eq, PartialEq)]
pub struct HoldEmTablePlayer {
    active: bool,
    hand: hand::HoldEmHand,
    stack: u32,
    position: u8,
}

pub struct HoldEmTable {
    dealer_location: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
    max_table_size: u8,
}
