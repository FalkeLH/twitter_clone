use chrono::{prelude::*, serde::ts_seconds};
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    handle: String,
}

impl Author {
    fn new(name: &str, handle: &str) -> Self {
        Self {
            name: name.to_string(),
            handle: handle.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Tweet {
    contents: String,
    author: Author,
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
}

impl Tweet {
    fn new(contents: &str, author: Author) -> Self {
        Self {
            contents: contents.to_string(),
            author,
            time: Utc::now(),
        }
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

fn main() {
    let author = Author::new("John Doe", "JDoe132");
    let tweet = Tweet::new("Hello, World!", author);
    let serialized_tweet = serde_json::to_string(&tweet).unwrap();
    println!("{}", tweet);
    println!("{}", serialized_tweet);
    let authors =
        return_authors("test.json").expect("Something went wrong with loading the authors");
    println!("{:#?}", authors);
}

fn return_authors<P: AsRef<Path> + Copy>(path: P) -> Result<Author, Box<dyn Error>> {
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
