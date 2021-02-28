// use std::process;
use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn data_new_test() {
        use crate::Data;
        let s = "1.0,2.1,0.3,0.0";
        let data = match Data::new(s){
            Data::Dense(v) => v,
            _ => vec![],
        };
        let ans = vec![1.0, 2.1, 0.3, 0.0];
        assert_eq!(data, ans);
    }
}


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
    pub fn new(filename: String) -> Result<Config, &'static str> {

        let kind = match FileKind::new(&filename) {
            Some(k) => k,
            None => return Err("No extension"),
        };

        Ok(Config{ kind, filename })
    }

}

// I'll implement these enum/struct to general type, some day.

#[derive(Debug)]
pub enum Data {
    Dense(Vec<f64>),
    Sparse(HashMap<u64, f64>),
}

impl Data {
    pub fn new(line: &str) -> Data {
        let data = line.split(',')
                       .map(|s| s.parse().unwrap())
                       .collect();
        Data::Dense(data)
    }
}


pub struct SupervisedData {
    pub label: i32,
    pub data: Data,
}

fn read_csv(filename: &String) -> Result<Vec<Data>, Box<Error>> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = contents.lines()
                          .map(|line| Data::new(line))
                          .collect();
    Ok(results)
}

pub fn read(mut s: String) -> Result<Vec<Data>, Box<Error>> {
    let config = Config::new(s)?;

    /*
    match config.kind {
        FileKind::CSV => read_csv(&config.filename),
        // FileKind::LIBSVM => read_libsvm(config.filename),
        _ => Err(Box::new("File format error")),
    }
    */
    read_csv(&config.filename)
}
