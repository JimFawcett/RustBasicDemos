/////////////////////////////////////////////////////////////
// file_io::main.rs - Test some std namespace facilities   //
//                                                         //
// Test:                                                   //
// - std::env::current_dir()                               //
// - std::path::Path                                       //
// - std::fs::File::open                                   //
// - std::String                                           //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::Result;
use display::{*};
// extern crate libc;
//use libc::w

//extern "C" { fn is_open(f:File) -> bool; }
// #[allow(dead_code)]
// fn is_open(f:File) -> bool {
//     unsafe {
//         f.descriptor >= 0
//     }
// }
#[allow(dead_code)]
fn open_file_for_write(file_name:&str) ->Result<File> {
    use std::fs::OpenOptions;
    let rfile = OpenOptions::new()
               .write(true)
               .create(true)
               .open(file_name);
    //rfile.share_mode(DENY_READ | DENY_WRITE | DENY_DELETE);
    //rfile.descriptor;
    rfile
}
#[allow(dead_code)]
fn open_file_for_read(file_name:&str) ->Result<File> {
    use std::fs::OpenOptions;
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

#[allow(dead_code)]
fn write_string_to_file(mut f:File, s:&str) -> Result<usize> {
    f.write(s.as_bytes())
}

#[allow(dead_code)]
fn read_file_to_string(mut f:File) -> Result<String> {
  let mut contents = String::new();
  let _bytes = f.read_to_string(&mut contents);
  Ok(contents)
}

#[allow(dead_code)]
fn checked_write_string_to_file(mut f:File, s:&str) -> bool {
    let rslt = f.write(s.as_bytes());
    let success = test_file_op(&rslt);

    if success {
      //print!("\n  write succeeded");
      return true;
    }
    else {
        print!("\n  Error: {}", rslt.err().unwrap());
        return false;
    }
}

#[allow(dead_code)]
fn test_file_op<T>(r:&Result<T>) -> bool {
     match &r {
        Err(why) => { print!("\n  {}", why);  return false },
        Ok(_) => return true
    };
}

fn main() -> std::io::Result<()> {

    let file_name = "first_test.txt";
    let file_text = "Howdy folks!";
    
    /*-- first write --*/
    let f = open_file_for_write(file_name);
    let success = test_file_op(&f);

    if success {
        print!("\n  successfully opened file \"{}\" for write", file_name);
        let rslt = write_string_to_file(f.unwrap(), file_text);
        if test_file_op(&rslt) {
            print!("\n  wrote \"{}\" to \"{}\"", file_text, file_name);
        }
        else {
            print!("\n  {}", rslt.err().unwrap());
        }
    }
    else {
        print!("\n  {}", f.err().unwrap());
    }
    putline();

    /*-- second write --*/
    let f = open_file_for_write(file_name);
    let success = test_file_op(&f);

    let file_text = "some other text";
    if success {
        print!("\n  successfully opened file \"{}\" for write", file_name);
        let ok = checked_write_string_to_file(f.unwrap(), file_text);
        if ok {
            print!("\n  checked_write of {:?} succeeded", file_text);
        }
        else {
            print!("\n  checked_write failed");
        }
    }
    else {
        print!("\n  {}", f.err().unwrap());
    }
    putline();

    /*-- first open for read --*/
    let rslt = open_file_for_read(&file_name);
    let ok = test_file_op(&rslt);
    match ok {
        true => print!("\n  open_file_for_read succeeded"),
        false => print!("\n  open_file_for_read failed"),
    };
    if ok {
        let s = read_file_to_string(rslt.unwrap()).unwrap();
        print!("\n  Read string {:?} from file {:?}", s, file_name);
    }
    else {
        print!("\n  read_file_to_string failed:\n    {:?}",rslt.err().unwrap());
    }
    // putline();
    // let f:File;
    // {
    //     let rslt = open_file_for_write(f);
    //     if is_open(f) {
    //         print!("\n  test file is open");
    //     }
    //     else {
    //         print!("\n  test file is closed");
    //     }
    // }
    // let rslt = open_file_for_write(f);
    // if is_open(f) {
    //     print!("\n  test file is open");
    // }
    // else {
    //     print!("\n  test file is closed");
    // }

    putlinen(2);

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
    let num:usize = 16;
    print!("\n  Read {} bytes", n);
    print!(", displaying {} bytes", num);
    print!("\n  The bytes:\n{:?}", &buffer[..num]);
    
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
