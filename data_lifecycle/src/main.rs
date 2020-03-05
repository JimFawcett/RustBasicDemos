/////////////////////////////////////////////////////////////
// data_lifecycle::main.rs - demonstrate object life cycle //
//                                                         //
// work in progress - not ready for prime time             //
// - doesn't yet do what I want to illustrate              //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

use display::*;  // import all pubs

#[allow(dead_code)]
fn show_copy() {
    print!("\n  copied");
}
#[derive(Debug)]
struct TestBlittable {
    pub id:u8,
}
impl Copy for TestBlittable { // copy trait has no methods
}                             // its a marker trait

impl Clone for TestBlittable {
    fn clone(&self) -> TestBlittable {
        print!("\n  cloned");
        *self
    }
}
impl TestBlittable {
    #[allow(dead_code)]
    pub fn new(id : u8) -> Self {
        Self {
            id,
        }
    }
}
fn main() {
   
    let x = TestBlittable::new(42);
    let mut y = x;
    print!("\n  y = {:?}", y);
    y = x.clone();
    let _z = x; // makes copy
    log(&x);    // so we can still use x
    // let cls = || print!("\n  a closure");
    // cls();
    // (|| print!("\n  another closure"))();
    print!("\n  y = {:?}\n\n", y);
}
