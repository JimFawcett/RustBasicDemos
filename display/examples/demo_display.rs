extern crate display;
use display::{*};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn test_displays() {
    main_title("demo display");
    sub_title("--  shows  --");
    shows("\n  showing type and value:");
    putline();
    sub_title("--  show_type and show_value  --");
    let mut str = String::new();
    str.push_str("a string");
    show_value(&str);
    putline();
    sub_title("--  log  --");
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
    sub_title("--  show --");
    show("\n  this is a Point structure\n  ", &point);
    putline();
    sub_title("that's all folks!");
    putline();
}

fn main() {
    test_displays();
}