use std::collections::HashMap;
use std::vec;

use crate::engine::bet;
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
    bet_history: Vec<bet::Bet>,
    dealer_location: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
    round: u8,
    max_table_size: u8,
}
