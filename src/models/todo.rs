use crate::schema::todos;
use crate::schema::todos::dsl::*;
use actix_web::web;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

type PgDbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub text: String,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "todos"]
pub struct UpdateTodo {
    pub id: i32,
    pub text: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "todos"]
pub struct NewTodo {
    pub text: String,
}

impl Todo {
    pub fn all(pool: web::Data<PgDbPool>) -> Result<Vec<Todo>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = todos.load::<Todo>(&conn)?;
        Ok(items)
    }

    pub fn create(pool: web::Data<PgDbPool>, item: NewTodo) -> Result<Todo, diesel::result::Error> {
        let conn = pool.get().unwrap();

        diesel::insert_into(todos)
            .values(&item)
            .execute(&conn)
            .expect("Error inserting todo");

        let mut items = todos
            .find(id)
            .load::<Todo>(&conn)
            .expect("Error loading a Todo");
        Ok(items.pop().unwrap())
    }

    pub fn update(
        pool: web::Data<PgDbPool>,
        update_todo: UpdateTodo,
    ) -> Result<Todo, diesel::result::Error> {
        let conn = pool.get().unwrap();

        let row = diesel::update(todos)
            .filter(id.eq(&update_todo.id))
            .set(text.eq(&update_todo.text))
            .get_result(&conn);

        Ok(row.unwrap())
    }

    pub fn delete(pool: web::Data<PgDbPool>, todo_id: i32) -> Result<usize, diesel::result::Error> {
        let conn = pool.get().unwrap();

        let count = diesel::delete(todos.find(todo_id)).execute(&conn)?;
        Ok(count)
    }
}
