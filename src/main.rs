use serde::{Serialize, Deserialize};
use chrono::{prelude::*, serde::ts_seconds};
use std::fmt::Display;

#[derive(Debug, Serialize)]
struct Tweet {
    contents: String,
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
}

impl Tweet {
    fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
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
    let tweet = Tweet::new("Hello, World!");
    let serialized_tweet = serde_json::to_string(&tweet).unwrap();
    println!("{}", tweet);
    println!("{}", serialized_tweet);
}
