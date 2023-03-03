//! main.rs

use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    println!("The server is listening on 127.0.0.1:{}", port);
    run(listener)?.await
	
}


