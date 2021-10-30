use actix_web::{http, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use kube::api::{
    admission::{AdmissionRequest, AdmissionResponse, AdmissionReview},
    DynamicObject,
};

#[post("/mutate")]
async fn handle_mutate(
    req: HttpRequest,
    body: web::Json<AdmissionReview<DynamicObject>>,
) -> impl Responder {
    tracing::info!("mutate endpoint handled");
    // some codes
    // send request,
    // create AdmissionResponse
    return HttpResponse::Ok().json("{\"message\":\"ok\"}");
}

#[get("/health")]
async fn health() -> impl Responder {
    tracing::info!("health endpoint handled");
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .json("{\"message\":\"ok\"}")
}

#[actix_rt::main]
pub async fn simple_addmission_webhook() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    HttpServer::new(|| {
        App::new()
            .service(handle_mutate)
            .service(health)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
