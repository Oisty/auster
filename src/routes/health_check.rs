use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn ready_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
