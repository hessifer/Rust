fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// holds data / definition
struct Server {
    addr: String,
}

// implementation
impl Server {
    // constructor for our struct
   fn new(addr: String) -> Self { // Self is an alias for Server
        Self {
            addr
        }
   }

  fn run(self) {
      println!("Listening on {}", self.addr);
  }
}
