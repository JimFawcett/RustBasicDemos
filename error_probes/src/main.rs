/////////////////////////////////////////////////////////////
// RustBasicDemos::error_probes                            //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 08 Mar 2020  //
/////////////////////////////////////////////////////////////

use std::io::prelude::*;
use std::fs::File;
use display::{*};

/*-- configure result --*/
fn demo_result<'a>(p: bool) -> Result<&'a str, &'a str> {
    print!("\n  value of input predicate is {}", p);
    if p {
        return Ok("it's ok");
    } 
    else {
        return Err("it's not ok");
    }
}

fn demo_option<'a>(p:bool) -> Option<&'a str> {
    print!("\n  value of input predicate is {}", p);
    if p {
        return Some("something just for you!");
    }
    else {
        return None;
    }
}

#[allow(dead_code)]
fn open_file_for_read(file_name:&str) ->Result<File, std::io::Error> {
    use std::fs::OpenOptions;
    let rfile = OpenOptions::new()
               .read(true)
               .open(file_name);
    rfile
}

#[allow(dead_code)]
use std::io::{Error, ErrorKind};
fn read_file_to_string(mut f:File) -> Result<String, std::io::Error> {
  let mut contents = String::new();
  let bytes_rslt = f.read_to_string(&mut contents);
  if bytes_rslt.is_ok() {
    Ok(contents)
  }
  else {
      Err(Error::new(ErrorKind::Other, "read error"))
  }
}

fn main() -> Result<(), &'static str> {

    sub_title("  -- demo Option and Result --  ");

    shows("\n-- using match");
    let r = demo_result(true);
    match r {
        Ok(rslt) => print!("\n    result is {}", rslt),
        Err(rslt) => print!("\n    result is {}", rslt)
    }
    let r = demo_result(false);
    match r {
        Ok(rslt) => print!("\n    result is {}", rslt),
        Err(rslt) => print!("\n    result is {}", rslt)
    }

    shows("\n\n-- using expect");
    let r = demo_result(true).expect("predicate was false");
    print!("\n    result is {}", r);
    /////////////////////////////////////////////
    // uncomment to see panic
    // let _r = demo_result(false).expect("predicate was false");
    putline();

    sub_title("  -- demo Option --  ");
    shows("\n--using match");

    let r = demo_option(true);
    match r {
        Some(rslt) => print!("\n    {}", rslt),
        None => print!("\n    sorry, nothing here")
    }
    let r = demo_option(false);
    match r {
        Some(rslt) => print!("\n    {}", rslt),
        None => print!("\n    sorry, nothing here")
    }

    shows("\n\n--using unwrap");
    let r = demo_option(true).unwrap();
    print!("\n    {}", r);
    /////////////////////////////////////////////
    // uncomment to see panic
    // let _r = demo_option(false).unwrap();
    putline();

    /*-----------------------------------------------------
       - Choose name of a file in the error_probes crate
         root directory to show successful operation.  
       - Choose one that does not exist to show failure 
         operation.
    */
    let file:File;
    let file_name = "foobar.txt";
    let rslt = open_file_for_read(file_name);
    if rslt.is_ok() {
      print!("\n  file {:?} opened successfully", file_name);
      file = rslt.unwrap();  // no panic because is_ok
      let s = read_file_to_string(file);
      if s.is_ok() {
          print!("\n  contents: \"{}\"", s.unwrap());  // no panic
      }
    }
    else {
        print!("\n  failed to open file {:?}", file_name);
    }

    shows("\n\n--error bubbling with ? operator");
    print!("\n  {:?}", demo_result(true)?);
    /////////////////////////////////////////////
    // uncomment statement below to see error
    // return from main, but no panic
    //-------------------------------------------
    // print!("\n  {:?}", demo_result(false)?);
    
    print!("\n\n  That's all folks!\n\n");
    Ok(())
}
