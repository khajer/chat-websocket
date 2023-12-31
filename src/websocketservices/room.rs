use std::collections::HashSet;

pub struct Room {
    members: HashSet<usize>,
    owner: usize,
}
