use std::collections::HashMap;

#[derive(Clone)]
pub struct RoomMgr {
    pub rooms: HashMap<String, Room>,
    pub cnt: u32,
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
}

impl RoomMgr {
    pub fn show(&mut self) {
        self.cnt += 1;

        println!("{}", self.cnt);
    }
}
pub fn new() -> RoomMgr {
    RoomMgr {
        rooms: HashMap::new(),
        cnt: 0,
    }
}
