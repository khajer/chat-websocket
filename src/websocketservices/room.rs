use super::player::Player;

pub struct Room {
    status: String,
    members: Vec<Player>,
    password: String,
}
