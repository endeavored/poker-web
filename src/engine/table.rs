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
    state: TableState,
    active_players: VecDeque<player::Player>,
    bet_history: Vec<bet::Bet>,
    board: Vec<card::Card>,
    dealer_location: u8,
    max_table_size: u8,
    player_list: HashMap<player::PlayerId, HoldEmTablePlayer>,
    pot: u32,
}

impl HoldEmTable {
    fn place_bet(&mut self, player_id: player::PlayerId, amount: u32) {
        let Some(user) = self.player_list.get_mut(&player_id) else {
            return;
        };
        if user.stack < amount {
            return;
        }
        user.stack -= amount;
        self.pot += amount;
        let user_bet = bet::Bet::new(player_id, amount, self.state);
        self.bet_history.push(user_bet);
    }
    fn _take_bets(&mut self) {} // TODO: Implement betting state machine
    fn _end_game(&mut self) {}
    fn _next_state(&mut self) {
        self._take_bets();
        match self.state {
            TableState::Preflop => {
                self.state = TableState::Flop;
            }
            TableState::Flop => {
                self.state = TableState::Turn;
            }
            TableState::Turn => {
                self.state = TableState::River;
            }
            TableState::River => {
                self._end_game();
                self.state = TableState::Preflop;
            }
        }
    }
}
