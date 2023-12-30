use actix_web::{HttpServer, App};
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(views::views_factory)
    }).bind("127.0.0.1:8080")?.run().await
}