use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub handle: String,
}

impl Author {
    pub fn new(name: &str, handle: &str) -> Self {
        Self {
            name: name.to_string(),
            handle: handle.to_string(),
        }
    }
}

pub fn return_authors<P: AsRef<Path> + Copy>(path: P) -> Result<Author, Box<dyn Error>> {
    let mut file = open_file_correctly(path)?;
    let mut contents = String::new();
    let file_len = file.read_to_string(&mut contents)?;
    if file_len == 0 {
        serde_json::to_writer(&mut file, &Author::new("test", "test"))
            .expect("Something went wrong with writing the test author");
    }

    let file = open_file_correctly(path)?;
    eprintln!("{}", contents);
    let reader = BufReader::new(file);
    let out = serde_json::from_reader(reader).expect("Cannot load from reader");
    Ok(out)
}

fn open_file_correctly<P: AsRef<Path> + Copy>(path: P) -> Result<File, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path);
    match file {
        Ok(f) => Ok(f),
        Err(e) => Err(Box::new(e))
    }
}
