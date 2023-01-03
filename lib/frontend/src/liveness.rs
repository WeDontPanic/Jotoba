use actix_web::HttpResponse;

pub async fn ready() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn healthy() -> HttpResponse {
    HttpResponse::Ok().finish()
}
