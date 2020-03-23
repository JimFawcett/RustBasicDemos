/////////////////////////////////////////////////////////////
// data_types::main.rs - demonstrate Rust data types       //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 15 Feb 2020  //
/////////////////////////////////////////////////////////////
/*
   Simple demonstrations of Rust data types:
   - ints: i32, i64, usize
   - floats: f32, f64
   - char
   - bool
   - unit ()
   - array: [1, 2, 3]
   - tuple: (1, 3.5,'Z')
   - String "a string".to_string()
   - struct: struct Point { x:f64, y:f64, z:f64, }
   - enum: enum POS { BS(String), MS(String), PhD(String), }
   - Vec<T>: Vec<i32>::new();
   - HashMap<Key, Value>:cl HashMap<String, int>::new();
*/
#[allow(unused_imports)]
use display::{
    putline, main_title, show_type, show_value, log, slog, putlinen
};
use std::fmt::{Debug, Display};
use std::any::Any;

#[allow(dead_code)]
fn put<T: Any + Debug + Display>(value: &T) {
    print!("{}", value);
}

fn putln<T: Any + Debug + Display>(value: &T) {
    let mut str_temp = String::new();
    str_temp.push_str("\n  ");
    str_temp.push_str(&value.to_string());
    print!("{}", str_temp);
}

fn separator() {
    put(&"\n ---------------------------------");  
}

fn main() {

    main_title("exploring basic types");
    /*
      Rust ints:
      i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    */
    let demo :i8 = 3;
    putln(&"let demo :i8 = 3;");
    log(&demo);

    separator();
    let demo = 5;
    putln(&"let demo = 5;");
    log(&demo);

    separator();
    let demo :usize = 7;
    putln(&"let demo :usize = 7;");
    log(&demo);

    /* Rust floats: f32, f64 */

    separator();
    let demo = 3.5;
    putln(&"let demo = 3.5;");
    log(&demo);

    separator();
    let demo :f32 = -3.5;
    putln(&"let demo :f32 = -3.5;");
    log(&demo);

    /* Rust chars: char */

    separator();
    let demo = 'a';
    putln(&"let demo = 'a';");
    log(&demo);

    separator();
    let demo :char = 'Z';
    putln(&"let demo :char = 'Z';");
    log(&demo);

    /* Rust boolean: bool */

    separator();
    let demo = true;
    putln(&"let demo = true;");
    log(&demo);

    separator();
    let demo :bool = false;
    putln(&"let demo :bool = false");
    log(&demo);

    /* Rust unit type: () */

    separator();
    let demo = ();
    putln(&"let demo = ();");
    log(&demo);

    separator();
    let demo :() = ();
    putln(&"let demo :() = ();");
    log(&demo);

    /* Rust arrays: [1,2,3] */

    separator();
    let demo = [1, 2, 3];
    putln(&"let demo = [1, 2, 3];");
    log(&demo);

    separator();
    let demo = [(1.5, 2.3), (3.1, 3.9)];
    putln(&"let demo = [(1.5, 2.3), (3.1, 3.9)];");
    log(&demo);

    /* Rust tuples: (1,'z',3.5) */
    
    separator();
    let demo = (1, 2.5, (1,'a'), [1,2,3]);
    putln(&"let demo = (1, 2.5, (1,'a'), [1,2,3]);");
    log(&demo);

    /* Rust Strings: String::from("a string") */
    
    separator();
    let demo = String::from("a demo String");
    putln(&"let demo = String::from(\"a demo String\");");
    log(&demo);

    /* Rust references:  */
    
    separator();
    let mut demo = String::from("a demo String");
    putln(&"let demo = String::from(\"a demo String\");");
    log(&demo);
    let rdemo = &mut demo;
    putln(&"let rdemo = &mut demo;");
    putln(&"redemo.push_str(\" more stuff\")");
    rdemo.push_str(" more stuff");
    // compile fails - this is attemp to make second
    // mutable borrow because push_str takes &self
    // demo.push_str(" still more stuff");
    log(rdemo);
    putln(&"demo.push_str(\" still more stuff\"");
    // this call succeeds only because rdemo is not used below call
    demo.push_str(" still more stuff");
    log(&demo);
    // log(rdemo);  // if this call is uncommented both calls above fail
    putline();

    /* attempt to mutate after borrow */

    let mut s = String::from("s is owner");
    log(&s);
    {
        let rs = &s;  // borrow s
        // statement below fails to compile
        // owner can't mutate after borrow 
        // s += " with stuff";
        log(&rs);
    }  // borrow ends here
    s += " with stuff";
    slog(&s);
    putline();
    
    /* Rust structs: struct Point { x:f64, y:f64, z:f64, t:} */
    
    separator();
    #[derive(Debug)]
    struct Point { x:f64, y:f64, z:f64, name:String, }
    let demo = Point { x:1.5, y:2.5, z:3.7, name:String::from("Peter") };
    putln(&"struct Point { x:f64, y:f64, z:f64, name:String, }");
    putln(&"let demo = Point { x:1.5, y:2.5, z:3.7, name:String::from(\"Peter\") };");
    log(&demo);

    /* Rust enums: enum POS { bs, ms, phd } */
    
    separator();
    #[derive(Debug)]
    #[allow(dead_code)]
    enum POS { BS(String), MS(String), PhD(String), };
    let demo = POS::MS(String::from("Computer Engineering"));
    putln(&"enum POS { BS(String), MS(String), PhD(String), };");
    putln(&"let demo = POS::MS(String::from(\"Computer Engineering\"));");
    log(&demo);

    /* Rust Vectors: Vec<i32> */
    
    separator();
    let mut demo  :Vec<(i32, f64, char)> = Vec::new();
    demo.push((1, 2.5, 'z'));
    demo.push((2, 3.5, 'A'));
    putln(&"let mut demo  :Vec<(i32, f64, char)> = Vec::new();");
    log(&demo);

    /* Rust HashMap: HashMap<String,i32> */
    
    separator();
    use std::collections::HashMap;
    let mut demo :HashMap<String, i32> = HashMap::new();
    demo.insert("one".to_string(), 1);
    demo.insert("two".to_string(), 2);
    putln(&"let mut demo :HashMap<String, i32> = HashMap::new();");
    log(&demo);
    demo.entry("three".to_string()).or_insert(4);
    show_value(&demo);
    demo.entry("three".to_string()).and_modify(|e| *e = 3);
    show_value(&demo);
    demo.entry("four".to_string()).or_insert(4);
    show_value(&demo);
    demo.entry("five".to_owned()).or_insert(6);
    show_value(&demo);
    demo.insert("five".to_string(), 5);
    show_value(&demo);
    if demo.contains_key("five") {
        show_value(&"has key five");
    }
    else {
        show_value(&"does not have key five");
    }

    putline();
    
    /* type aliases */

    separator();
    type PointF = (f64, f64, f64);
    type VecPoint = Vec<PointF>;
    let vp: VecPoint = vec![(1.0, 1.5, 2.0), (1.0, -1.5, 2.0)];
    log(&vp);
    putlinen(2);
}
