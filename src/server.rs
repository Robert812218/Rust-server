mod server {
  pub struct Server {
    addr: String,
  }
  
  impl Server {
    pub fn new(addr: String) -> Server {
      Self { addr }
    }
  
    pub fn run(self) {
      println!("Listening on {}", self.addr);
    }
  }
  
}