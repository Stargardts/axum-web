use axum::{
    extract::Query,
    http::{self, HeaderValue, StatusCode},
    routing::{get, post},
    Json, Router,
};
use mime_guess::from_path;
use serde::{Deserialize, Serialize};
mod encrypt;
use encrypt::{encrypt, generate_key,};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { handle_files("home").await }))
        .route(
            "/:file",
            get(
                |axum::extract::Path(file): axum::extract::Path<String>| async move {
                    handle_files(&file).await
                },
            ),
        )
        .route("/users", get(root))
        .route(
            "/users",
            // Use extracted username and userid to create a new user
            post(create_user),
        )
        .route("/width", post(log_width))
        .route("/userInput", post(handle_user_input));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// Handle file requests
async fn handle_files(file: &str) -> (StatusCode, http::HeaderMap, Vec<u8>) {
    let path = WebPages::from_path(file).path();
    if path == WebPages::FOF.path() {
        println!("404 Not Found: {}", file);
    }
    let content_type = from_path(path).first_or_octet_stream().to_string();
    let content_type: &str = &content_type;
    let content_length = std::fs::metadata(path).unwrap().len();
    // let cache_control = "max-age=180";
    let file = std::fs::read(path).unwrap();

    let mut headers = http::HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str(content_type).unwrap());
    headers.insert(
        "Content-Length",
        HeaderValue::from_str(&content_length.to_string()).unwrap(),
    );
    // headers.insert("Cache-Control", HeaderValue::from_str(cache_control).unwrap());
    (StatusCode::OK, headers, file)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Query(payload): Query<User>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: payload.id,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct Width {
    width: u64,
}

async fn log_width(Json(..): Json<Width>) -> (StatusCode, Json<Status>) {
    let response = Status {
        status: "ok".to_string(),
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize, Deserialize)]
struct Status {
    status: String,
}

#[derive(Serialize, Deserialize)]
struct Chat {
    response: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
struct UserInput {
    input: String,
}

async fn handle_user_input(Json(payload): Json<UserInput>) -> (StatusCode, Json<Chat>) {
    println!("User input: {}", payload.input);
    let key = generate_key();
    let (_, ciphertext) = encrypt(&key, &payload.input);
    let response = Chat {
        response : ciphertext,
        key: hex::encode(key),
    };
    (StatusCode::OK, Json(response))
}

enum WebPages {
    Home,
    About,
    Style,
    Pic,
    Chat,
    Script,
    FOF,
}

impl WebPages {
    fn from_path(path: &str) -> Self {
        match path {
            "home" => WebPages::Home,
            "about" => WebPages::About,
            "style" => WebPages::Style,
            "pic" => WebPages::Pic,
            "chat" => WebPages::Chat,
            "script" => WebPages::Script,
            _ => WebPages::FOF,
        }
    }

    fn path(&self) -> &'static str {
        match self {
            WebPages::Home => "webpages/index.html",
            WebPages::About => "webpages/about.html",
            WebPages::Style => "webpages/styles/style.css",
            WebPages::Pic => "webpages/images/pic.jpg",
            WebPages::Chat => "webpages/chat.html",
            WebPages::Script => "webpages/scripts/script.js",
            WebPages::FOF => "webpages/404.html",
        }
    }
}
