use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open	("pg26.txt").unwrap();
    let mut buffer = Vec::new();
    // f.read_to_end(&mut f).
    f.read_to_end(&mut buffer).unwrap();
}
