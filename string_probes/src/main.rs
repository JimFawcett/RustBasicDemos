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

fn put<T>(t:T) where T:Display {
  print!("{}", t);
}

fn putdb<T>(t:T) where T:Debug {
  print!("{:?}", t);
}

fn put_str(s:&String) {
  print!("{}",s);
}
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
/*-- note: order n, from vectorize -- prefer at, above --*/
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

    let ch:u8 = 'a' as u8;
    log(&ch);
    show("char is ", &(ch as char));
    putline();

    /*-- String --*/

    let s:String = String::from("Rust");
    log(&s);
    let i:usize = 2;
    let ch = at(&s, i);
    print!("\n  in string \"{}\", char at {} is {}", &s, i, ch);
    show("length in bytes of s = {:?}", &s.len());
    putline();

    let s1 = s.clone();
    let v:Vec<u8> = Vec::from(s1);
    log(&v[0]);
    show("vec from string",&v);
    putline();

    /*----------------------------------------------------- 
       Note that Windows does not display utf-8 correctly
       in any of its terminals, e.g., cmd, x64 Native Tools,
       Powershell, ..., so these will only display correctly
       on Linux systems, and apple??
    */
    let mut s2 = String::new();
    s2.push_str("\u{1F600}");
    s2.push('\u{1F601}');
    s2.push('\u{1F602}');
    s2.push('\u{1F609}');
    print!("\n  {}", s2);
    putline();
    print!("\n  {}", '\u{1F601}');
    
    /*-- str --*/

    let s_slice = &s[..];   // slice containing all chars of s
    log(&s_slice);
    show("s_slice = ", &s_slice);
    putline();

    let s_slice2 = s.as_str();
    log(&s_slice2);
    putline();

    /*-- create string and mutate --*/

    let mut s = String::new();
    s.push('a');
    s.push(' ');
    s.push_str("test string");
    log(&s);
    putline();

    let t = s.replace("string","Rust String");
    log(&t);
    putline();

    for tok in s.split_whitespace() {
      print!("\n  {}", tok);
    }
    putline();

    //-----------------------
    // this works too
    // let iter = s.split_whitespace();
    // for tok in iter {
    //   print!("\n  {}", tok);
    // }
    // putline();

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
        log(&slice_all);
        show("slice_all = ", &slice_all);
        putline();

        let third = &s[2..3];       // string slice with one char
        log(&third);
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
        // log(&slice_all);

    }   // elem borrow ends here

    s.push('Z');  // ok, borrows no longer active
    putline();

    /* format_args! macro */

    let s = std::fmt::format(format_args!("\n  {}, {}, {}", 1, 2, 3.5));
    put_str(&s);
    put(&s);
    putlinen(1);
    let s = String::from("\n  1, 2, -3.5");
    put(&s);
    log(&s);
    show("s = ", &s);

    put("\n  ");
    putdb(&(1, 2.5, 'a'));

    put("\n  ");
    putdb(&(1, -2.5, 'a'));

    put("\n  ");
    putdb(&{1; 2.5; 'z'});

    #[derive(Debug)]
    struct S {x:i32, y:f64, s:String, };
    let st:S = S { x:3, y:4.2, s:"xyz".to_string() };
    put("\n  ");
    putdb(&st);
    putline();

    sub_title("That's all Folks!");
    putlinen(2);
}
