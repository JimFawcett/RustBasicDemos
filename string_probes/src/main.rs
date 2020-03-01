/////////////////////////////////////////////////////////////
// string_probes::main.rs - basic string operations        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Feb 2020  //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use display::{ 
  log, slog, show, show_type, show_value, 
  putline, putlinen, main_title, sub_title 
};
#[allow(unused_imports)]
use std::fmt::{ Debug, Display };
///////////////////////////////
// for later
// #[allow(unused_imports)]
// use std::ffi::OsString;

/*-----------------------------------------------------------
   Note:
   Strings hold utf8 characters, which vary in size, so you
   you can't directly index String instances.
*/
#[allow(dead_code)]
pub fn at(s:&String, i:usize) -> char {
  s.chars().nth(i).unwrap()
}
/*-----------------------------------------------------------
   note: 
   - order n, as str chars are utf8, e.g., from 1 to 5 bytes
   - this ugliness is one way to index
   - see below for another, not much better way
*/
#[allow(dead_code)]
pub fn vectorize(s: &str) -> Vec<char> {
  s.chars().collect::<Vec<char>>()
}
/*-- note: order n, from vectorize --*/
#[allow(dead_code)]
pub fn get_char(s:&str, i:usize) -> char {
    vectorize(s)[i]
}
/*-- stringize - order n --*/
#[allow(dead_code)]
pub fn stringize(v: &Vec<char>) -> String {
  return v.into_iter().collect()
}

fn main() {

    main_title("string_probes");
    putline();

    /*-- char --*/

    let v:Vec<char> = vec!['R', 'u', 's', 't'];
    log(&v);
    log(&'R');
    putline();

    /*-- String --*/
    let ch:u8 = 'a' as u8;
    log(&ch);
    show("char is ", &(ch as char));
    putline();

    let s:String = String::from("Rust");
    log(&s);
    let i:usize = 2;
    let ch = at(&s, i);
    print!("\n  in string \"{}\", char at {} is {}", &s, i, ch);
    putline();

    let s1 = s.clone();
    let v:Vec<u8> = Vec::from(s1);
    slog(&v[0]);
    show("vec from string",&v);

    /*-- str --*/
    let s_slice = &s[..];
    slog(&s_slice);
    show("s_slice = ", &s_slice);
    putline();

    /*-- create string and mutate --*/
    let mut s = String::new();
    s.push('a');
    s.push(' ');
    s.push_str("test string");
    log(&s);
    putline();

    /*-----------------------------------------------------
       Another, order n, way to index string:
      - chars returns iterator over utf8 chars in string slice
      - nth(i) calls next on iterator until it gets to i
      - nth(i) returns std::option::Option<char>:
         - that contains Some(ch) or None if operation failed
    */
    let result = s.chars().nth(0);
    match result {
      Some(r) => show("s.chars().nth(0) = ", &r),
      None => print!("\n  couldn't extract char"),
    }
    show("s = ", &s);
    let result = s.chars().nth(2);
    match result {
      Some(r) => show("s.chars().nth(2) = ", &r),
      None => print!("\n  couldn't extract char"),
    }
    show("s = ", &s);
    putline();

    {
        /*-------------------------------------------------
           Caution here: 
           - slice is returning array of bytes, not utf8 chars
           - this works only because we use all ASCII chars
        */
        /*-- slices are non-owning views and are borrows of s --*/
        let slice_all = &s;
        slog(&slice_all);
        show("slice_all = ", &slice_all);
        putline();

        let third = &s[2..3];       // string slice with one char
        slog(&third);
        show("third = ",&third);
        putline();

        /*-- this works for utf-8 encoding --*/
        let ch = third.chars().nth(0);  // 
        log(&ch);
        match ch {
          Some(x) => { log(&x); show("match ch = ", &x); },
          None => print!("\n can't return ch"),
        }
        
        ///////////////////////////////////////////////////
        // compile fails 
        // - can't modify owner while borrows are active
        //------------------------------------------------
        // s.push('Z');
        // slog(&slice_all);

    }   // elem borrow ends here

    s.push('Z');  // ok, borrows no longer active
   
    putline();
    sub_title("That's all Folks!");
    putlinen(2);
}
