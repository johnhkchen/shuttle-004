use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use shuttle_actix_web::ShuttleActixWeb;
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")] 
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
    let hello = HelloTemplate { name: "Dude" };
    HttpResponse::Ok().body(hello.render().unwrap())
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello);
    };

    Ok(config.into())
}
