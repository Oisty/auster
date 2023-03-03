use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Redirect;

pub(crate) async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    Redirect::to("/").using_status_code(StatusCode::FOUND)
}

pub(crate) async fn logout(id: Identity) -> impl Responder {
    id.logout();

    Redirect::to("/").using_status_code(StatusCode::FOUND)
}