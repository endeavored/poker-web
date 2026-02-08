use chrono::{DateTime, Utc};

pub struct Bet {
    player_id: PlayerId,
    amount: u32,
    timestamp: DateTime<Utc>,
}
