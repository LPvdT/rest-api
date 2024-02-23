use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
mod todolist;

#[allow(dead_code)]
pub(self) struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(self) struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
pub(crate) async fn index() -> String {
    let r = HttpResponse::Ok().finish().status();
    format!("{}: This is a health check.", r)
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8080));

    server?.run().await
}
