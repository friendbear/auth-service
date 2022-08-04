#[macro_use]
extern crate diesel;
extern crate serde_json;

mod models;
mod vars;
mod invitation_handler;
mod utils;
mod errors;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use actix_web::{middleware, web, App, HttpServer};

    std::env::set_var("RUST_LOG",
        "actix_server=info, actix_web=info, simple-auth-server=debug");
    env_logger::init();


    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(db::get_connection_pool())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(vars::domain().as_str())
                    .max_age(86400) // one day in seconds
                    .secure(false), // this can only be true if you have https
            ))
            // limit the maximum amount of data that server will accept
            .data(web::JsonConfig::default().limit(4096))
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("*")
            //         .allowed_methods(vec!["GET", "POST", "DELETE"])
            //         .max_age(3600)
            // )
            // everything under '/api/' route
            .service(
                web::scope("/api")
                .service(
                    web::resource("/invitation")
                        .route(web::post().to(invitation_handler::post_invitation)),
                )
                // .service(
                //     web::resource("/register/{invitation_id}")
                //         .route(web::post().to(invitation_handler::register_user)),
                // )
                // .service(
                //     web::resource("/auth")
                //         .route(web::post().to(auth_handler::login))
                //         .route(web::delete().to(auth_handler::logout))
                //         .route(web::get().to(auth_handler::get_me)),
                // )
            )
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
