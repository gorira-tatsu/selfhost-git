use std::hash::{Hash, Hasher};
use std::str::Bytes;
use sha1::{Sha1, Digest};
use std::fs::File;


fn main() {
    let mut hasher = Sha1::new();
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

    let blob_string = "blob ".to_owned() + &blob.content_length.to_string() + "\0" + &blob.content;
    let blob_b = blob_string.as_bytes();
    
    hasher.update(blob_b);
    let result = hasher.finalize();

    println!("blob {}\0{}", blob.content_length, blob.content);
    println!("{}" , hex::encode(result));
}