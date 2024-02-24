use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080));

    server?.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    let res = HttpResponse::Ok().finish().status();

    HttpResponse::Ok().json(HealthCheckDto {
        code: res.into(),
        msg: "This is a health check".to_string(),
    })
}

pub struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntryDto>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoListEntryDto {
    id: i32,
    date: i64,
    title: String,
}

#[derive(Serialize)]
pub struct HealthCheckDto {
    code: u16,
    msg: String,
}
