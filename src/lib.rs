use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Signature of `run` changes.
// It return `Server` on the happy path and `async` keyword was dropped.
// There is no more call to `.await`.
pub fn run(address: &str) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind(address)?
    .run();

    Ok(server)
}
