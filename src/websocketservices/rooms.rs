use std::collections::HashMap;

#[derive(Clone)]
pub struct Rooms {
    rooms: HashMap<String, String>,
}

impl Rooms {
    pub fn count() -> u32 {
        0
    }
}
pub fn new() -> Rooms {
    Rooms {
        rooms: HashMap::new(),
    }
}
