use actix::{Actor, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, Result};
use actix_web_actors::ws;

use serde::Serialize;

#[derive(Serialize)]
struct Version {
    version: String,
}

#[get("/version")]
async fn version() -> Result<impl Responder> {
    let version = Version {
        version: "0.0.1".to_string(),
    };
    Ok(web::Json(version))
}

struct MyWs;
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                println!("byte");
            }
            Ok(ws::Message::Text(text)) => {
                println!("byte message");
                ws_message(ctx, text.to_string());
            }
            Ok(ws::Message::Binary(bin)) => {
                print!("Binary");
                ctx.binary(bin);
            }
            _ => (),
        }
    }
}

fn ws_message(ctx: &mut ws::WebsocketContext<MyWs>, text: String) {
    println!("receive: '{}'", text);
    if text == "" {
        ctx.text("text1");
    } else {
        ctx.text("text2");
    }
}
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);

    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(version)
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
