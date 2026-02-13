use actix_web::{web, App, HttpServer, Responder};

async fn overview() -> impl Responder {
    format!("<h1>Welcome to the Pollinations AI Bot</h1><p>This is the overview page for the Pollinations AI Bot!</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(overview))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}