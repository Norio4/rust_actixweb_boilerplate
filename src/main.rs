#[macro_use] extern crate diesel_migrations;
extern crate rust_actixweb_boilerplate as app;

use crate::app::util::{establish_connection, init_pool};
use crate::app::routes;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::http::header;
use actix_cors::Cors;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {

    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    run_migrations();

    let bind = "0.0.0.0:3000";

    HttpServer::new(move || {
        App::new()
            .configure(routes::routes)
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://example.com")
                    .allowed_methods(vec!["OPTIONS", "HEAD", "GET", "PATCH", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .data(establish_connection())
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}

pub fn run_migrations() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url).unwrap();

    let conn = pool.get().unwrap();
    embed_migrations!();
    embedded_migrations::run(&conn).expect("migration error");
}
