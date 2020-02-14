////////////////////////////////////////////////////////////
// try_std.rs - Test some of the std namespace facilities
//
// Test:
// - std::env::current_dir()
// - std::path::Path
// - std::fs::File::open
// - std::String

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
//use std::any;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let path = Path::new("src/test.txt");
    println!("The path to open is {}", path.display());
    let filename = format!("{}", path.display()); 
    let mut f = File::open(filename).expect("can't open file");

    /////////////////////////////////////////////////
    // This works too:
    // let mut f2 = File::open("src/test.txt")?;

    let mut buffer = [0; 150];
    
    // read up to 150 bytes
    let n = f.read(&mut buffer)?;
    println!("Read {} bytes", n);
    println!("The bytes: {:?}", &buffer[..n]);
    
    let mut str = String::new();
    str.push_str("  ");
    for i in 0..n {
        let ch = buffer[i] as char;
        if ch != '\n' {
            str.push(ch);
        }
        else {
            str.push(ch);
            str.push_str("    ");
        }
    }
    println!("\n  Buffer as chars: \n  {}", str);
    Ok(())
}
