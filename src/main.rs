extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

mod models;
mod vars;

#[macro_use]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    use actix_cors::Cors;
    use actix_files::Files;
    use actix_session::CookieSession;
    use actix_web::{middleware, web, App, HttpServer, http::header};
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use diesel::{
        prelude::*,
        r2d2::{self, ConnectionManager}
    };

    std::env::set_var("RUST_LOG",
        "actix_server=info, actix_web=info, simple-auth-server=debug");
    env_logger::init();

    // create a database connection pool
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Faild to create a database connection pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // Enable sessions
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(vars::domain().as_str())
                    .max_age(86400)
                    .name("auth")
                    .secure(false),
                )
            .wrap(
                Cors::default()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .max_age(3600)
            )
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to(invitation_handler::postinvitation)),
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(invitation_handler::register_user)),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(auth_handler::login))
                            .route(web::delete().to(auth_handler::logout))
                            .route(web::get().to(auth_handler::get_me)),
                    )
                )
//                Files::new("/assets", "./templates/assets"))
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
