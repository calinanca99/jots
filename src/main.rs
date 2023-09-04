// The project exposes the following routes:
// - POST /jwt_token; exchange credentials for JWT token
// - GET /book/{name}/{author}; looks up a book by book name and
// author
// - GET /books; get all the books saved by the user
// - POST /save_book/{id}; saves a new book
// - DELETE /delete_book/{id}; deletes a book
//
// A valid JWT token must be sent with each request, except
// POST /jwt_token and GET /book/{name}/{author}.

use axum::{response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new().route("/ping", get(ping));
    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Cannot parse address"))
        .serve(app.into_make_service())
        .await
        .expect("Cannot start server");

    Ok(())
}

async fn ping() -> impl IntoResponse {
    "Pong!\n"
}
