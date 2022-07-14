use actix_web::HttpResponse;

pub(crate) async fn health_check() -> HttpResponse {
    println!("/health_check");
    HttpResponse::Ok().finish()
}
