use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub struct Server {
    total: u16,
}

impl Server {
    pub fn new() -> Server {
        Server { total: 0 }
    }
}

impl Actor for Server {
    type Context = actix::Context<Server>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Start Thread Server");
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Server {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("In comming Server");
    }
}
