use std::collections::HashSet;

struct Table {
    playerList: HashSet<i32>, // This will be the player IDs
    maxTableSize: uint8,
}
