//Including methods from standard library
use std::fs::File;
use std::io::Read;

fn main() {
    //unwrap() says don't compile if there's an issue
    let mut f = File::open("pg26.txt").unwrap();
    // Vec is a data structure that is dynamic and stored sequentially
    let mut buffer = Vec::new();
    // "unpack" the file to the vec
    f.read_to_end(&mut buffer).unwrap();
    // print out the word count
    println!("There are {} words in your text.", buffer.len());
}
