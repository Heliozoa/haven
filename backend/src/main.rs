//! Haven

use axum::{
    http::{HeaderValue, Method, StatusCode},
    routing, Extension, Router,
};
use diesel::r2d2::ConnectionManager;
use haven::{
    routes::{auth, posts, users},
    ConnectionPool,
};
use hyper::header::CONTENT_TYPE;
use std::env;
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be defined");

    // ensure static dirs exist
    std::fs::create_dir_all("./static/asset").expect("Failed to create ./static/asset");
    std::fs::create_dir_all("./static/image").expect("Failed to create ./static/image");
    std::fs::create_dir_all("./static/avatar").expect("Failed to create ./static/avatar");
    std::fs::create_dir_all("./static/thumbnail").expect("Failed to create ./static/thumbnail");

    let conn = ConnectionManager::new(&database_url);
    let pool = ConnectionPool::new(conn).expect("Failed to create connection pool");

    let key = Key::from(secret_key.as_bytes());

    let app = Router::new()
        // static files
        .nest(
            "/static",
            routing::get_service(ServeDir::new("static")).handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandler internal error: {}", error),
                )
            }),
        )
        // api
        .nest(
            "/api",
            Router::new()
                // auth
                .route("/auth/logged", routing::get(auth::logged))
                .route("/auth/login", routing::post(auth::login))
                .route("/auth/logout", routing::post(auth::logout))
                .route("/auth/register", routing::post(auth::register))
                // comics
                // lists
                // posts
                .route("/posts", routing::get(posts::list).post(posts::new))
                .route("/posts/:id", routing::get(posts::view))
                .route("/posts/:id/comment", routing::post(posts::comment))
                .route(
                    "/posts/:id/tag/:id",
                    routing::put(posts::tag).delete(posts::untag),
                )
                // tags
                // users
                .route("/users/:id", routing::get(users::view).patch(users::update))
                .route("/users/:id/follow", routing::put(users::follow))
                .route("/users/:id/unfollow", routing::put(users::unfollow))
                .route("/users/:id/avatar", routing::put(users::upload_avatar)),
        )
        // layers
        .layer(CookieManagerLayer::new())
        .layer(Extension::<ConnectionPool>(pool))
        .layer(Extension::<Key>(key))
        .layer(
            CorsLayer::new()
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers(vec![CONTENT_TYPE])
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                    "http://localhost:1234",
                )))
                .allow_credentials(true),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
