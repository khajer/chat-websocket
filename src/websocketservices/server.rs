use actix::prelude::*;
use actix::{Actor, Message, StreamHandler};
use actix_web_actors::ws::{self};

#[derive(Message)]
#[rtype(result = "()")]
pub struct LOBBY {
    pub name: String,
}

pub struct WSServer {}

impl WSServer {
    pub fn new() -> WSServer {
        WSServer {}
    }
}

impl Actor for WSServer {
    type Context = actix::Context<WSServer>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Start Thread Server");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSServer {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        println!("In comming Server");
    }
}

impl Handler<LOBBY> for WSServer {
    type Result = ();
    fn handle(&mut self, msg: LOBBY, _: &mut Context<Self>) -> Self::Result {
        println!("Received: {}", msg.name)
    }
}
