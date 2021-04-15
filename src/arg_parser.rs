use std::process::exit;

use crate::print::print::error;

pub fn arg_parser(args: &[String]) -> (&str, &str) {
  if args.len() == 3 {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
  } else {
    error("error", "please provide a tag");
    exit(123);
  }
}
