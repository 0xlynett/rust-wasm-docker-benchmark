use actix_web::{App, HttpServer, Responder, get, web};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    "Roses are red
Violets are blue
Unexpected '{' at line 32

(source unknown)"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}, what's up?", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("open on port 3030");
    HttpServer::new(|| App::new().service(index).service(json).service(hello))
        .bind(("0.0.0.0", 3030))?
        .run()
        .await
}

#[derive(Deserialize)]
struct Info {
    username: String,
    place: String,
    lucky_number: u64,
}

#[get("/json")]
async fn json(info: web::Json<Info>) -> impl Responder {
    format!(
        "Welcome {} from {}! Your lucky unsigned integer is {}",
        info.username, info.place, info.lucky_number
    )
}
