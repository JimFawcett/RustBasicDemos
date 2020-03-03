/////////////////////////////////////////////////////////////
// string_probes::main.rs - basic string operations        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Feb 2020  //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use display::{ log, show, putline, putlinen, main_title, sub_title };
#[allow(unused_imports)]
use std::fmt::{ Debug, Display };
#[allow(unused_imports)]
use std::ffi::OsString;

/*
   Note:
   Strings hold utf8 characters, which vary in size, so you can't
   directly index String instances.
*/
// fn car_cdr(s: &str) -> (&str, &str) {
//     for i in 1..5 {
//         let r = s.get(0..i);
//         match r {
//             Some(x) => return (x, &s[i..]),
//             None => (),
//         }
//     }

//     (&s[0..0], s)
// }
/*-- note: order n, as str chars are utf8 ---*/
#[allow(dead_code)]
fn vectorize(s: &str) -> Vec<char> {
  s.chars().collect::<Vec<char>>()
}
/*-- note: order n, from vectorize --*/
#[allow(dead_code)]
fn get_char(s:&str, i:usize) -> char {
    vectorize(s)[i]
}
/*-- stringize - order n --*/
fn stringize(v: &Vec<char>) -> String {
  return v.into_iter().collect()
}

fn main() {

    main_title("string_probes");
    putline();

    /*-- create string and mutate --*/
    let mut s = String::new();
    s.push('a');
    s.push(' ');
    s.push_str("test string");
    log(&s);

    {
        /*-- borrow element of slice of s --*/
        //let slice_all = &s;
        let third = &s[2..3];      // string slice with one char
        log(&third);
        let ch = third as char;    // extracting
        log(ch);
        
        print!("\n  third = {}", ch);
        let mut v = vectorize(&s);
        v[2] = '\u{1F60A}';
        s = stringize(&v);
        show("modified s = ", &s);
        
        let oss:OsString = OsString::from("this is an OsString");
        show("oss = ", &oss);


        // let elem = &mut getChar(&s, 2);
        // log(elem);
        // *elem = 'a';
        // log(elem);
        // show("s = ",&s);

        /////////////////////////////////////////////
        // fails to compile - second mutable borrow
        // v.push(4);
        // log(&v);
        // log(elem);  // without this, will compile

    }   // elem borrow ends here

    // v.push(4);
    // log(&v);
    // putline();

    // let v = vec![5, 4, 3, 2, 1];  // original v dropped
    // show("v = ", &v);
    
    // let vc = v.clone();
    // show("v clone() = ", &vc);

    // let vs = &v[1..3];
    // show("v slice = &v[1..3] = ", &vs);

    /////////////////////////////////////////////////
    // Equivalent:
    //-----------------------------------------------
    // let s = String::from("v slice = &v[1..3] = ");
    // show(&s, &vs);
    
    /////////////////////////////////////////////////
    // Demo index out of bounds:
    //-----------------------------------------------
    // std::env::set_var("RUST_BACKTRACE", "1");
    // log(&v[5]);  // panic - out of bounds index
   
    putline();
    sub_title("That's all Folks!");
    putlinen(2);
}
