//mod types;
use std::fmt::{Debug};

#[allow(dead_code)]
pub fn run () {

    /*-- fully specified --*/
    let i:i32 = 5;
    let f:f64 = 3.4;
    let a:[f32; 5] = [1.0, 1.5, 2.0, 1.5, 1.0];
    let t:(i32, f64, String) = (1, 2.0, "three".to_string());
    #[derive(Debug)]
    struct S{i:i32, s:&'static str, };
    let s:S = S{i:15, s:"a literal string" };
    #[derive(Debug)]
    enum E {BS(String), MS(String), PhD(String),};
    let e:E = E::MS("Computer Engineering".to_string());

    print!("\n  -- fully specified types --\n");
    print!("\n  i = {:?}", i);
    print!("\n  f = {:?}", f);
    print!("\n  a = {:?}", a);
    print!("\n  t = {:?}", t);
    print!("\n  s = {:?}", s);
    print!("\n  e = {:?}", e);

    /*-- using type deduction --*/
    let i = 5;
    let f = 3.4;
    let a = [1.0, 1.5, 2.0, 1.5, 1.0];
    let t = (1, 2.0, "three".to_string());
    let s = S{i:15, s:"a literal string" };
    let e = E::MS("Computer Engineering".to_string());

    print!("\n\n  -- using type deduction --\n");
    print!("\n  i = {:?}", i);
    print!("\n  f = {:?}", f);
    print!("\n  a = {:?}", a);
    print!("\n  t = {:?}", t);
    print!("\n  s = {:?}", s);
    print!("\n  e = {:?}", e);
}