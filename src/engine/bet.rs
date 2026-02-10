use chrono::{DateTime, Utc};

use crate::engine::player::PlayerId;
use crate::engine::table::TableState;

#[derive(Debug, Clone)]
pub struct Bet {
    pub player_id: PlayerId,
    pub amount: u32,
    pub table_round: TableState,
    timestamp: DateTime<Utc>,
}

impl Bet {
    pub fn new(player_id: PlayerId, amount: u32, table_round: TableState) -> Self {
        Self {
            player_id,
            amount,
            table_round,
            timestamp: Utc::now(),
        }
    }
}
