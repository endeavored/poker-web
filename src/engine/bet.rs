use chrono::{DateTime, Utc};

use crate::engine::player::PlayerId;

#[derive(Debug, Clone)]
pub struct Bet {
    pub player_id: PlayerId,
    pub amount: u32,
    timestamp: DateTime<Utc>,
}
