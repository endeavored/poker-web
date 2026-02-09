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
    pub stack: u32,
    position: u8,
}

pub struct HoldEmTable {
    active_players: LinkedList<player::Player>,
    bet_history: Vec<bet::Bet>,
    board: Option<[card::Card; 5]>,
    dealer_location: u8,
    max_table_size: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
    round: u8,
}

impl HoldEmTable {
    fn place_bet(&mut self, player_id: player::PlayerId, amount: u32) {
        if !self.player_list.contains_key(&player_id) {
            return;
        }
        let user = self.player_list.get_mut(&player_id).unwrap();
        if user.stack < amount {
            return;
        }
        user.stack -= amount;
        self.pot += amount;
        let user_bet = bet::Bet::new(player_id, amount, self.round);
        self.bet_history.push(user_bet);
    }
}
