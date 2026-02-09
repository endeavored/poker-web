use std::collections::HashMap;
use std::collections::LinkedList;

use crate::engine::bet;
use crate::engine::card;
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
    active_player: LinkedList<player::Player>,
    bet_history: Vec<bet::Bet>,
    board: Option<[card::Card; 5]>,
    dealer_location: u8,
    max_table_size: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
    round: u8,
}
