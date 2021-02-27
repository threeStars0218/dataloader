use std::env;
use std::process;

#[derive(Debug)]
enum FileKind {
    CSV,
    LIBSVM,
}

impl FileKind {
    fn new(filename: &str) -> Option<FileKind> {

        let pos = match filename.rfind('.') {
            Some(p) => p,
            None => return None,
        };

        let kind = match &filename[pos..] {
            ".csv" => FileKind::CSV,
            ".libsvm" => FileKind::LIBSVM,
            _ => return None,
        };

        Some(kind)
    }
}

#[derive(Debug)]
pub struct Config {
    kind: FileKind,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file specified"),
        };

        let kind = match FileKind::new(&filename) {
            Some(k) => k,
            None => return Err("No extension"),
        };

        Ok(Config{ kind, filename })
    }
}

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Hello, world!");
    println!("{:?}", config);
}
