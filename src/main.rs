use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, web,web::Redirect, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
};

const ONE_MINUTE: Duration = Duration::minutes(1);

async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    Redirect::to("/").using_status_code(StatusCode::FOUND)
}

async fn logout(id: Identity) -> impl Responder {
    id.logout();

    Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let secret_key = Key::generate();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::post().to(logout)))
            .service(web::resource("/").route(web::get().to(index)))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}