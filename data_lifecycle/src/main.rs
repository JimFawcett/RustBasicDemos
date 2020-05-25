/////////////////////////////////////////////////////////////
// data_lifecycle::main.rs - demonstrate object life cycle //
//                                                         //
// work in progress - not ready for prime time             //
// - doesn't yet do what I want to illustrate              //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

use display::*;  // import all pubs
//use std::io::*;

#[allow(dead_code)]
fn show_copy() {
    print!("\n  copied");
}
#[derive(Debug)]
struct Dropper;
impl Drop for Dropper {
    fn drop(&mut self) {
        print!("\n  Dropper instance dropped");
        // let _ = std::io::stdout().flush();
    }
}
#[derive(Debug)]
struct TestBlittable {
    pub id:u8,
}
impl Copy for TestBlittable { // copy trait has no methods
}                             // it's a marker trait

impl Clone for TestBlittable {
    fn clone(&self) -> TestBlittable {
        print!("\n  cloned");
        Self {
            id: self.id,
        }
    }
}
// can't implement both copy and drop
//-------------------------------------
// impl Drop for TestBlittable {
//     fn drop(&mut self) {
//         print!("\n  dropping instance of TestBlittable");
//     }
// }
impl TestBlittable {
    #[allow(dead_code)]
    pub fn new(id : u8) -> Self {
        Self {
            id,
        }
    }
}
#[derive(Debug)]
struct TestNonBlittable {
    name: String,
}
impl Clone for TestNonBlittable {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
        }
    }
}
impl Drop for TestNonBlittable {
    fn drop(&mut self) {
        print!("\n  dropping TestNonBlittable instance {:?}", self.name);
    }
}
impl TestNonBlittable {
    fn new(name: &str) -> TestNonBlittable {
        TestNonBlittable {
            name: name.to_string(),
        }
    }
    fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
fn main() {
    print!("\n  Demonstrate object lifetimes");
    print!("\n ==============================");
    let putline = || print!("\n");

    let _dropper = Dropper;
    //drop(dropper);
    let x = TestBlittable::new(42);
    let mut y = x;
    print!("\n  y = {:?}", y);
    y = x.clone();
    print!("\n  y = {:?}", y);
    let z = x; // makes copy
    print!("\n  z = {:?}", z);
    log(&x);    // so we can still use x
    putline();

    let a = TestNonBlittable::new("tnb_a");
    print!("\n  TestNonBlittable: {:?}", a);
    let mut b = a.clone();
    b.name("tnb_b");
    print!("\n  TestNonBlittable: {:?}", b);
    let c = &b;
    print!("\n  TestNonBlittable: {:?}", c);

    //-------------------------------------------
    // statement below fails to compile
    // - can't move out of reference
    // - would leave dangling reference
    // let d = *c;

    let mut d = a;  // move ok - no refs
    d.name("tnb_d");
    print!("\n  TestNonBlittable: {:?}", d);

    //-------------------------------------------
    // statement below won't compile
    // - can't use a, invalid since a moved to d
    // print!("\n  TestNonBlittable: {:?}", a);

    let cls = || print!("\n  a closure");
    cls();
    (|| print!("\n  another closure"))();
    putline();
}
