extern crate dataloader;

use std::env;
use std::process;

use dataloader::read;

fn main() {

//     let config = Config::new(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
// 
//     println!("{:?}", config);

    let filename = String::from("test.csv");
    let v = read(filename);
    for data in v.iter() {
        println!("{:?}", data);
    }
    println!("Hello, world!");
}
