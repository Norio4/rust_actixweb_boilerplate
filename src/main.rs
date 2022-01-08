#[macro_use]
extern crate diesel_migrations;
extern crate rust_actixweb_boilerplate as app;

use crate::app::routes;
use crate::app::util::{establish_connection, init_pool};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web;
use actix_web::{http, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use log::info;

use crate::app::config_initializer::ConfigInitializer;

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    run_migrations();

    let port = std::env::var("APP_PORT").expect("APP_PORT must be set");
    let bind = format!("0.0.0.0:{}", port);

    info!("Start Config Initializer");
    let config_file = "config/app.yml";
    let ci = ConfigInitializer {
        config_file: config_file.to_string(),
    };
    let app_config = ci.initialize();
    let cors_vec: Vec<String> = ci.get_cors_origins();

    HttpServer::new(move || {
        let cors: Cors = cors_vec.iter().fold(Cors::default(), |cors, origin| {
            cors.allowed_origin(origin)
                .allowed_methods(vec!["OPTIONS", "HEAD", "GET", "POST"])
                .allowed_header(http::header::CONTENT_TYPE)
                .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                .supports_credentials()
        });

        App::new()
            .configure(routes::routes)
            .app_data(web::Data::new(app_config.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(establish_connection())
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}

pub fn run_migrations() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url).unwrap();

    let conn = pool.get().unwrap();
    embed_migrations!();
    embedded_migrations::run(&conn).expect("migration error");
}
