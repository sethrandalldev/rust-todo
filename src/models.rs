use diesel::{Queryable, Insertable, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::todos;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
}