mod author;
use author::*;

mod tweet;
use tweet::*;

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


