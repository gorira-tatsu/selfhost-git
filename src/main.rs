use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::Read;

fn main() {
    struct BlobObject {
        content: String,
        content_length: usize,
    }

    let mut blob = BlobObject {
        content: String::new(),
        content_length: 0,
    };
    blob.content = std::fs::read_to_string("test.txt").unwrap();
    blob.content_length = blob.content.len();

    println!("Content: {}", blob.content);
    println!("Content Length: {}", blob.content_length);
}