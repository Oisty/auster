use actix_web::web;
use crate::common;

use crate::handlers::{auth};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/login").route(web::post().to(auth::login)))
            .service(web::resource("/logout").route(web::post().to(auth::logout)))
            .service(web::resource("/").route(web::get().to(common::index)))
    );
}