use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
struct Tweet {
    contents: String,
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

fn main() {
    let tweet = Tweet {
        contents: String::from("Hello, World!"),
    };
    let serialized_tweet = serde_json::to_string(&tweet).unwrap();
    println!("{}", tweet);
    println!("{}", serialized_tweet);
}
