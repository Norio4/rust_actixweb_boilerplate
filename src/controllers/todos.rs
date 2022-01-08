use actix_web::{web, HttpResponse, ResponseError};
use askama::Template;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};
use thiserror::Error;

type PgDbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::models::todo::NewTodo;
use crate::models::todo::Todo;
use crate::models::todo::UpdateTodo;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for CustomError {}

struct TodoItem {
    id: i32,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddParams {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    id: i32,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteParams {
    id: i32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    items: Vec<TodoItem>,
}

pub async fn index(pool: web::Data<PgDbPool>) -> Result<HttpResponse, CustomError> {
    let todo_list = Todo::all(pool).unwrap();

    let mut items = Vec::new();
    for todo in todo_list {
        items.push(TodoItem {
            id: todo.id,
            text: todo.text.to_string(),
        });
    }

    let html = IndexTemplate { items };

    let resposne_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(resposne_body))
}

// JSON case // pub async fn create(_pool: web::Data<PgDbPool>, params: web::Json<AddParams>) -> Result<HttpResponse, CustomError> {
pub async fn create(
    pool: web::Data<PgDbPool>,
    params: web::Form<AddParams>,
) -> Result<HttpResponse, CustomError> {
    println!("posted_todo");
    println!("{:?}", params);

    let new_text = &params.text;
    let new_todo = NewTodo {
        text: new_text.to_string(),
    };

    let _output_todo = Todo::create(pool, new_todo).unwrap();
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}

pub async fn update(
    pool: web::Data<PgDbPool>,
    params: web::Form<UpdateParams>,
) -> Result<HttpResponse, CustomError> {
    println!("posted_todo");
    println!("{:?}", params);

    let update_todo = UpdateTodo {
        id: params.id,
        text: params.text.to_string(),
    };

    let _output_todo = Todo::update(pool, update_todo).unwrap();
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}

pub async fn delete(
    pool: web::Data<PgDbPool>,
    params: web::Form<DeleteParams>,
) -> Result<HttpResponse, CustomError> {
    println!("posted_todo");
    println!("{:?}", params);

    let _output_todo = Todo::delete(pool, params.id).unwrap();
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}
