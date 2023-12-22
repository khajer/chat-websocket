use actix::{Actor, Addr};
use actix_web::{
    get,
    web::{self},
    Error, HttpRequest, HttpResponse, Responder, Result,
};
use actix_web_actors::ws;
use serde::Serialize;
use websocketservices::server::Server;

mod websocketservices;

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

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        websocketservices::session::Session {
            session_id: "".to_string(),
            name: "".to_string(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = websocketservices::server::Server::new().start();
    use actix_web::{App, HttpServer};
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .app_data(web::Data::new(server.clone()))
            .service(version)
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
