extern crate ansi_term;

use std::env;
use std::fs;

mod arg_parser;
use arg_parser::arg_parser;

mod print;
use print::print::done;
use print::print::error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = arg_parser(&args);
    match query {
        "minify" => {
            let file: String = fs::read_to_string(filename).expect("not a file");
            let file = file.replace("\n", "");
            fs::write(filename, file).expect("sorry");
            done(query);
        }
        "destroy" => {
            fs::remove_file(filename).expect("sorry");
            done(query);
        }
        "empty" => {
            fs::write(filename, "").expect("sorry");
            done(query);
        }
        "read" => {
            let file: String = fs::read_to_string(filename).expect("not a file");
            println!("\n {} \n", file);
            done(query);
        }
        "minify-new" => {
            let file: String = fs::read_to_string(filename).expect("not a file");
            let file = file.replace("\n", "");
            let mut l: Vec<&str> = filename.split(".").collect();
            let b = l[1];
            l[1] = "-minified.";
            l.push(b);
            let new_file: String = l.join("");
            fs::write(new_file, file).expect("sorrry");
            done(query);
        }
        _ => error(query, "this is not a valid tag"),
    }
}
