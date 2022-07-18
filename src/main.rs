use std::fmt::Display;

struct Tweet {
    contents: String
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

fn main() {
    let tweet = Tweet { contents: String::from("Hello, World!") };
    println!("{}", tweet);
}
