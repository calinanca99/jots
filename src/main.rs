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

pub const KEY_PAIR: [u8; 64] = [
    240, 110, 82, 228, 224, 211, 161, 191, 92, 249, 101, 114, 184, 219, 251, 72, 125, 157, 20, 178,
    5, 191, 135, 1, 125, 97, 152, 70, 201, 25, 190, 165, 124, 171, 27, 2, 80, 58, 203, 160, 126,
    121, 149, 210, 223, 43, 153, 92, 17, 119, 28, 184, 1, 182, 243, 108, 31, 131, 32, 24, 143, 118,
    150, 213,
];

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jwt_simple::prelude::*;
use serde::Deserialize;

// User stuff
pub type Users = HashMap<UserCredentials, UserId>;
pub type UserBooks = HashMap<UserId, HashSet<BookId>>;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct UserId(pub u64);

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserClaims {
    pub books: Vec<Book>,
}

// Book stuff
pub type Books = HashMap<BookId, Book>;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct BookId(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub author: String,
}

// App state
struct AppState {
    users: Users,
    user_books: UserBooks,
    books: Books,
    key_pair: Ed25519KeyPair,
}

impl AppState {
    fn users() -> Users {
        HashMap::from([(
            UserCredentials {
                username: "username".to_string(),
                password: "password".to_string(),
            },
            UserId(1),
        )])
    }

    fn user_books() -> HashMap<UserId, HashSet<BookId>> {
        HashMap::from([(UserId(1), HashSet::from([BookId(1), BookId(2), BookId(3)]))])
    }

    fn books() -> Books {
        HashMap::from([
            (
                BookId(1),
                Book {
                    name: "Book 1".into(),
                    author: "Author 1".into(),
                },
            ),
            (
                BookId(2),
                Book {
                    name: "Book 2".into(),
                    author: "Author 2".into(),
                },
            ),
            (
                BookId(3),
                Book {
                    name: "Book 3".into(),
                    author: "Author 3".into(),
                },
            ),
        ])
    }
}

impl Default for AppState {
    fn default() -> Self {
        let key_pair = Ed25519KeyPair::from_bytes(&KEY_PAIR).expect("Cannot create JWT Pair");

        Self {
            users: Self::users(),
            user_books: Self::user_books(),
            books: Self::books(),
            key_pair,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/jwt_token", post(jwt_token))
        .with_state(state);
    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Cannot parse address"))
        .serve(app.into_make_service())
        .await
        .expect("Cannot start server");

    Ok(())
}

async fn ping() -> impl IntoResponse {
    "Pong!\n"
}

/// Exchange credentials for a JWT Token.
async fn jwt_token(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<UserCredentials>,
) -> impl IntoResponse {
    let app_state = state.lock().unwrap();

    // Get the user
    match app_state.users.get(&payload) {
        Some(user_id) => {
            // Get user books
            let user_books = get_user_books(&app_state.user_books, &app_state.books, user_id);

            // Create JWT Token that has a `books` claim based on `user_books`
            let claims = Claims::with_custom_claims(
                UserClaims { books: user_books },
                Duration::from_mins(2),
            )
            .with_subject(user_id.0.to_string())
            .with_issuer("books-service");

            let token = app_state
                .key_pair
                .sign(claims)
                .expect("Cannot sign JWT token");

            format!("{token}\n")
        }
        None => "Not Authorized\n".to_string(),
    }
}

fn get_user_books(user_books: &UserBooks, books: &Books, user_id: &UserId) -> Vec<Book> {
    match user_books.get(user_id) {
        Some(book_ids) => {
            let ub = book_ids.iter().fold(vec![], |mut bs: Vec<Book>, book_id| {
                if let Some(book) = books.get(book_id) {
                    bs.push(book.clone());
                    bs
                } else {
                    bs
                }
            });
            ub
        }
        None => vec![],
    }
}
