use smol::block_on;
pub fn test_async() {
  println!("----------test_async-------");
  block_on(hello());
}

async fn hello() {
  println!("hello");
} 