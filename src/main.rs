use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use self::models::*;
mod models;
mod schema;
use diesel::r2d2::Pool;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = "postgres://postgres:password@localhost/rust_todo";
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

type DbPool = Pool<ConnectionManager<PgConnection>>;

async fn get_todos(pool: web::Data<DbPool>,) -> impl Responder {
    use self::schema::todos::dsl::*;
    let mut connection = pool.get().expect("couldn't get db connection from pool");
    let results = todos
        .limit(5)
        .select(Todo::as_select())
        .load(&mut connection)
        .expect("Error loading todos");

    HttpResponse::Ok().json(results)
}

async fn get_todo(pool: web::Data<DbPool>, path: web::Path<uuid::Uuid>,) -> impl Responder {
    use self::schema::todos::dsl::*;
    let mut connection = pool.get().expect("couldn't get db connection from pool");

    let todo_id = path.into_inner();

    let results = todos
        .filter(id.eq(todo_id))
        .select(Todo::as_select())
        .first(&mut connection)
        .optional()
        .expect("Error loading todos");

    match results {
        None => { HttpResponse::NotFound().body("Todo not found") }
        Some(todo) => { HttpResponse::Ok().json(todo) }
    }

}

async fn create_todo(pool: web::Data<DbPool>, item: web::Json<NewTodoRequest>,) -> impl Responder {
    use self::schema::todos::dsl::*;
    let mut connection = pool.get().expect("couldn't get db connection from pool");

    let new_todo = NewTodo {
        id: uuid::Uuid::new_v4(),
        title: item.title.clone(),
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(&mut connection)
        .expect("Error saving new todo");

    HttpResponse::Ok().body("Todo created")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos", web::post().to(create_todo))
            // .route("/todos/{id}", web::put().to(update_todo))
            // .route("/todos/{id}", web::delete().to(delete_todo))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Todo App with Actix and Diesel")
}
