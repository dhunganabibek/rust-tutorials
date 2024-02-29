use axum::{routing::{get,post}, Json, Router};



#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(|| async { "Hello, world!" }))
    .route("/foo", get(get_foo));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  axum::serve(listener, app).await.unwrap();
  println!("Hello, world!")
}



async fn get_foo() -> Json<String> {
  return Json("foo".to_string());
}