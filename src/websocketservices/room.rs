use std::collections::HashSet;

pub enum RoomStatus {
    WAIT,
    START,
    END,
}

pub struct Room {
    pub members: HashSet<usize>,
    pub owner: usize,
    pub status: RoomStatus,
}

impl Room {
    pub fn new() -> Room {
        Room {
            members: HashSet::new(),
            owner: 0,
            status: RoomStatus::WAIT,
        }
    }
}
