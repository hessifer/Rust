use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("Failed to bind to random port");
    // bubble up the io::Error if we failed to bind the address
    // otherwise call .await on our Server
    run(listener)?.await
}
