use super::author::*;

use serde::{Deserialize, Serialize};
use chrono::{prelude::*, serde::ts_seconds};

use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub contents: String,
    pub author: Author,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
}

impl Tweet {
    pub fn new(contents: &str, author: Author) -> Self {
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
