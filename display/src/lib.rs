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
        title("test types".to_string());
        let mut str = String::new();
        str.push_str("a string");
        log(&str);
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
    }
}


/////////////////////////////////////////////////////////////
// display::main.rs - Demonstrate display types            //
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

/*------------------------------------------------------------- 
   log type name and value to console
   - expects T to implement Debug
   - see #[define(Debug)] attributes, above 
*/
pub fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
    let name = type_name::<T>();
    println!("TypeId: {}", name);

    // Try to convert our value to a `String`. If successful, we want to
    // output the String`'s length as well as its value. If not, it's a
    // different type: just print it out unadorned.
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!(" value: String ({}): {}\n", as_string.len(), as_string);
        }
        None => {
            println!(" value: {:?}\n", value);
        }
    }
}

// This function wants to log its parameter out prior to doing work with it.
// fn do_work<T: Any + Debug>(value: &T) {
//     log(value);
//     // ...do some other work
// }

/*-------------------------------------------------------------
   Display underlined title on console
*/
pub fn title(msg: String) {
    println!("  {}", msg);
    let s = std::iter::repeat('-').take(msg.len() + 2).collect::<String>();
    println!(" {}\n", s);
}
/*-- push a single newline to console --*/

pub fn putline() {
    println!("\n");
}
/*-- pust n newlines to console --*/

pub fn putlinen(n: usize) {
    let s = std::iter::repeat('\n').take(n).collect::<String>();
    println!("{}", s);
}

