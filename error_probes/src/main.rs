/////////////////////////////////////////////////////////////
// RustBasicDemos::error_probes                            //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 08 Mar 2020  //
/////////////////////////////////////////////////////////////

use display::{*};

fn demo_result<'a>(p: bool) -> Result<&'a str, &'a str> {
    print!("\n  value of input predicate is {}", p);
    if p {
        return Ok("it's ok");
    } 
    else {
        return Err("not ok");
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

fn main() {

    sub_title("  -- demo Result --  ");
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

    print!("\n\n  That's all folks!\n\n");
}
