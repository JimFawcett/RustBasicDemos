/////////////////////////////////////////////////////////////
// display_test::main.rs - test display library            //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 Feb 2020  //
/////////////////////////////////////////////////////////////
/*
  Usually you test a library using the library's built in
  test functions.  But those cannot write to the console
  because they are a compiled part of the library which
  has no console.

  So I've constructed this separate main that uses the
  library just like any other application.
*/

#[allow(unused_imports)]
use display::{ 
  putline, main_title, sub_title, 
  log, show_type, show_value, putlinen 
};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
/* Need more tests for added functions */

fn main() {
    main_title("test types");
    let mut str = String::new();
    str.push_str("a string");
    log(&str);
    show_type(&str);
    show_value(&str);
    let an_i8: i8 = 100;
    log(&an_i8);
    let mut vi : Vec<i32> = Vec::new();
    vi.push(-1);
    vi.push(0);
    vi.push(1);
    log(&vi);
    #[derive(Debug)]
    enum Test { Test1, Test2, };
    log(&Test::Test1);
    log(&Test::Test2);
    let point = Point { x:1.0, y:1.5, z:2.0 };
    log(&point);
    putline();
    sub_title("That's all folks!");
    putlinen(2);
}
