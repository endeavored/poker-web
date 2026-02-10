use std::collections::HashMap;
use std::collections::VecDeque;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TableState {
    Preflop,
    Flop,
    Turn,
    River,
}

pub struct HoldEmTable {
    _state: TableState,
    active_players: VecDeque<player::Player>,
    bet_history: Vec<bet::Bet>,
    board: Option<[card::Card; 5]>,
    dealer_location: u8,
    max_table_size: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
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
        let user_bet = bet::Bet::new(player_id, amount, self._state);
        self.bet_history.push(user_bet);
    }
    fn _take_bets(&mut self) {} // TODO: Implement betting state machine
    fn _end_game(&mut self) {}
    fn _next_state(&mut self) {
        let cur_state = self._state;
        self._take_bets();
        match cur_state {
            TableState::Preflop => {
                self._state = TableState::Flop;
            }
            TableState::Flop => {
                self._state = TableState::Turn;
            }
            TableState::Turn => {
                self._state = TableState::River;
            }
            TableState::River => {
                self._end_game();
                self._state = TableState::Preflop;
            }
        }
    }
}
