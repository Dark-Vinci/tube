use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> io::Result<()> {
    
    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    f.read(&mut buffer)?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    // read into a String, so that you don't need to do the conversion.
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;


    let mut buffer = [0; 10];

    // read exactly 10 bytes
    f.read_exact(&mut buffer)?;

    let f = BufReader::new(File::open("foo.txt")?);

    for byte in f.bytes() {
        println!("{}", byte.unwrap());
    }

    // and more! See the other methods for more details.
    Ok(())
}
