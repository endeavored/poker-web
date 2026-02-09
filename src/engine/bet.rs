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
    pub fn new(player_id: PlayerId, amount: u32) -> Self {
        Self {
            player_id: player_id,
            amount: amount,
            table_round: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
