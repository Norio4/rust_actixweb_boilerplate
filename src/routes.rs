use crate::controllers::todos;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(todos::index));
    cfg.route("/", web::post().to(todos::create));
    cfg.route("/delete", web::post().to(todos::delete));
}
