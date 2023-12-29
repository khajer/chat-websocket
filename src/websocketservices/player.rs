use actix::Addr;

use super::session::Session;

pub struct Player {
    name: String,
    pub addr: Addr<Session>,
    score: i32,
}
