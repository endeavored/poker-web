use std::collections::HashSet;

use crate::engine::player;

pub struct Table {
    player_list: HashSet<player::PlayerId>,
    max_table_size: u8,
}
