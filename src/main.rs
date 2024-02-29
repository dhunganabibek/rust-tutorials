use axum::{routing::get, Json, Router};
use sqlx::mysql::MySqlPoolOptions;


#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect("mysql://root:RisinG123%23@localhost:3306/test")
    .await.unwrap();
  
  let row: (i64,) = sqlx::query_as("SELECT ?")
    .bind(42_i64)
    .fetch_one(&pool)
    .await
    .unwrap();

  dbg!(row);

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