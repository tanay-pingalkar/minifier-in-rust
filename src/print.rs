pub mod print {
  pub use ansi_term::Color::{Blue, Red};
  pub fn done(query: &str) -> () {
    println!("{}:{}", query, Blue.bold().paint("done"));
  }
  pub fn error(query: &str, msg: &str) -> () {
    println!("{}:{}", query, Red.italic().paint(msg));
  }
}
