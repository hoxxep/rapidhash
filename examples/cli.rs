use std::io::Read;

/// Command-line tool for rapidhash.
///
/// # Usage
/// Reading stdin:
/// ```shell
/// echo "example" | cargo run --example cli
/// 8543579700415218186
/// ```
///
/// Reading file:
/// ```bash
/// cargo run --example cli -- example.txt
/// 8543579700415218186
/// ```
pub fn main() {
    let hash_arg = std::env::args().nth(1);

    let buffer = match hash_arg {
        None => {
            let mut buffer = Vec::with_capacity(1024);
            std::io::stdin().read_to_end(&mut buffer).expect("Could not read from stdin.");
            buffer
        }
        Some(filename) => {
            std::fs::read(filename).expect("Could not load file.")
        }
    };

    let hash = rapidhash::rapidhash(&buffer);
    println!("{hash}");
}
