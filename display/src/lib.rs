/////////////////////////////////////////////////////////////
// display::lib.rs - Demonstrate display types             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 Feb 2020  //
/////////////////////////////////////////////////////////////
/*
   log and do_work are derived from:
   https://doc.rust-lang.org/beta/std/any/index.html

*/
use std::fmt::Debug;
use std::any::Any;
use std::any::type_name;
use std::mem::size_of;

/*-----------------------------------------------
   Accepts either String or str
   - no automatic newline
*/
pub fn shows<S: Into<String>>(s:S) {
    print!("{}",s.into());
}
/*-------------------------------------------------------------
   Display message and value on console
   - no automatic newline
*/
pub fn show<T: Debug>(msg:&str, t:&T) {
    print!("{}{:?}", msg, t);
}
/*------------------------------------------------------------- 
   show value on console
   - expects T to implement Debug
*/
pub fn show_value<T: Debug>(value: &T) {
    print!("\n  value: {:?}", value);
}
/*------------------------------------------------------------- 
   show type name on console
*/
pub fn show_type<T>(_value: &T) {
    let name = std::any::type_name::<T>();
    print!("\n  TypeId: {}, size: {}", name, size_of::<T>());
}
  /*------------------------------------------------------------- 
   show type name and value on console
   - expects T to implement Debug
   - see #[define(Debug)] attributes, above 
*/
pub fn log<T: Debug>(value: &T) {
    let name = type_name::<T>();
    print!("\n  TypeId: {}, size: {}", name, size_of::<T>());
    print!("\n  value:  {:?}", value);
  }
  
/*------------------------------------------------------------- 
   log type name and value to console
   - expects T to implement Debug
*/
pub fn slog<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
    let name = type_name::<T>();
    print!("\n  TypeId: {}, size: {}", name, size_of::<T>());

    // Try to convert our value to a `String`. If successful, we want to
    // output the String`'s length as well as its value. If not, it's a
    // different type: just print it out unadorned.
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            print!("\n  value:  String ({}): {}", as_string.len(), as_string);
        }
        None => {
            print!("\n  value:  {:?}", value);
        }
    }
}
/*-------------------------------------------------------------
   Display underlined main title on console
*/
pub fn main_title(msg: &str) {
    print!("\n  {}", msg);
    let s = std::iter::repeat('=').take(msg.len() + 2).collect::<String>();
    print!("\n {}", s);
}
/*-------------------------------------------------------------
   Display underlined sub title on console
*/
pub fn sub_title(msg: &str) {
    print!("\n  {}", msg);
    let s = std::iter::repeat('-').take(msg.len() + 2).collect::<String>();
    print!("\n {}", s);
}
/*------------------------------------------------------------- 
   show line with len hyphens
*/
pub fn separator(len:u8) {
    let mut s = String::new();
    for _i in 1..len+2 { s.push('-');}
    print!("\n {}",s);
}
/*-------------------------------------------------------------
   push a single newline to console
*/
pub fn putline() {
    print!("\n");
}
/*-------------------------------------------------------------
   pust n newlines to console
*/
pub fn putlinen(n: usize) {
    let s = std::iter::repeat('\n').take(n).collect::<String>();
    print!("{}", s);
}

///////////////////////////////////////////////////////////////
// display::main.rs tests - Demonstrate display types        //
//                                                           //
// Jim Fawcett, https://JimFawcett.github.io, 14 Feb 2020    //
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
        z: f64,
    }
    #[test]
    /*
       Library doesn't write to console, so all this tests is that
       no panic occurred.  See test_display for useful tests.
    */
    fn test_types() {
        main_title("test types");
        let mut str = String::new();
        str.push_str("a string");
        shows("\n  showing type and value:");
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
        assert_eq!(1, 1);
        sub_title("that's all folks!");
    }
}
