use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, Result};
use actix_web_actors::ws;
use serde::Serialize;

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

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        websocketservices::wsservice::MyWs {
            clients_vec: vec![],
        },
        &req,
        stream,
    );

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
