/////////////////////////////////////////////////////////////
// debugformats.rs - printing floats with various formats  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io               //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use std::fmt::{self, Formatter, Display};
use std::collections::HashMap;

// https://doc.rust-lang.org/std/fmt/index.html
// https://cheats.rs/#formatting-strings

#[allow(dead_code)]
pub fn run() {
    
    print!("\n  -- debug formats --\n");
    
    // display integer
    let demo :i32 = 3;
    print!("\n  demo :i32 : value = {:?}", demo);    

    // display float
    let demo :f64 = 3.1415927;
    print!("\n  demo :f64 : value = {:?}", demo);    

    // display str
    let demo = "this is a literal string";
    print!("\n  demo : value = {:?}", demo);    

    // display String
    let demo :String = "this is a String instance".to_string();
    print!("\n  demo : value = {:?}", demo);    

    // display array
    let demo :[i32;5]  = [1, 2, 3, 2, 1];
    print!("\n  demo :[i32;5] : value = {:?}", demo);    

    // display tuple
    let demo :(i32, f64, &str)  = (1, 1.5, "abc");
    print!("\n  demo :(i32, f64, str) : value = {:?}", demo);    

    // display Vec<T>
    let demo :Vec<f32>  = vec![1.0, 1.5, 2.0];
    print!("\n  demo :Vec<f32> : value = {:?}", demo);    

    // display HashMap<K,V>
    let demo :HashMap<&str, i32>  = [("one", 1), ("two", 2), ("three", 3)].iter().cloned().collect();
    print!("\n  demo :HashMap<&str, int> :\n        value = {:?}", demo);    

    // display struct
    #[derive(Debug)]  // this is required for debug display - uses compiler generated defaults
    struct S {
        i: i32,
        f: f64,
        s: String
    }

    let demo :S = S { i:3, f:3.5, s:"three point five".to_string(), };
    print!("\n  demo :S : value = {:?}", demo);    
    
}