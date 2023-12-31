use std::collections::HashSet;

pub enum RoomStatus {
    WAIT,
    START,
    END,
}

pub struct Room {
    members: HashSet<usize>,
    owner: usize,
    status: RoomStatus,
}
