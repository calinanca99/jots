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

fn main() {
    println!("Hello, world!");
}
