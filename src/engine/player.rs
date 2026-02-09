#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PlayerId(u32);

pub struct Player {
    bankroll: u32,
}
