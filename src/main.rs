#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};


mod models;
mod schema;
mod vars;
mod invitation_handler;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {


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
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            // Enable sessions
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(vars::domain().as_str())
                    .max_age()
                    .secure(false),
                )
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
                            .route(web::post().to(invitation_handler::post_invitation)),
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to_async(register_handler::register_user)),
                    ),
                    //.service(
                    //    web::resource("/register/{invitation_id}")
                    //        .route(web::post().to(invitation_handler::register_user)),
                    //)
                    // .service(
                    //     web::resource("/auth")
                    //         .route(web::post().to(auth_handler::login))
                    //         .route(web::delete().to(auth_handler::logout))
                    //         .route(web::get().to(auth_handler::get_me)),
                    // )
                )
//                Files::new("/assets", "./templates/assets"))
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
