use chrono::{DateTime, Utc};

use crate::engine::player::PlayerId;

#[derive(Debug, Clone)]
pub struct Bet {
    pub player_id: PlayerId,
    pub amount: u32,
    pub table_round: u8,
    timestamp: DateTime<Utc>,
}

impl Bet {
    pub fn new(player_id: PlayerId, amount: u32, table_round: u8) -> Self {
        Self {
            player_id,
            amount,
            table_round,
            timestamp: Utc::now(),
        }
    }
}
