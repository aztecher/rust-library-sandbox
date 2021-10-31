use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_text() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn get_json() -> impl Responder {
    HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}")
}

#[actix_rt::main]
pub async fn simple_http_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_text))
            .route("/json", web::get().to(get_json))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
