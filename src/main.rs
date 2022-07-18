#[allow(unused_imports)] // This is just temporary.  
use serde::{Serialize, Deserialize};
use chrono::{prelude::*, serde::ts_seconds};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    handle: String
}

impl Author {
    fn new(name: &str, handle: &str) -> Self {
        Self {
            name: name.to_string(),
            handle: handle.to_string()
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
            time: Utc::now()
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
}
