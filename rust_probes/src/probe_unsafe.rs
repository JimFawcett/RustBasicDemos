/////////////////////////////////////////////////////////////
// probe_unsafe.rs - demo use of pointers                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 22 Mar 2020  //
/////////////////////////////////////////////////////////////

use std::fmt::{ Debug };
use display::{*};

trait Show : Debug {  // trait fns are public
    fn show(&self) {
        print!("\n  {:?}", &self);
    }
}
#[derive(Debug)]
struct Test { x:i32, y:f64, }
impl Show for Test {}
impl Test {
    pub fn new() -> Self {
        Self {
            x:42,
            y:1.5,
        }
    }
}

#[allow(dead_code)]
pub fn run () {

    sub_title("exploring struct layout with safe pointers");
    let mut t = Test::new();
    t.show();
    shows("\n  Note: Test implements traits Show and Debug\n");
    let rt = &t as *const Test;
    let rx = &t.x as *const i32;
    let ry = &mut t.y as *mut f64;
    let sz = std::mem::size_of::<Test>();
    let sx = std::mem::size_of::<i32>();
    let sy = std::mem::size_of::<f64>();
    print!("\n  address of t   = {:?}", rt as i32);
    print!("\n  address of t.x = {:?}", rx as i32);
    print!("\n  address of t.y = {:?}", ry as i32);
    print!("\n  size of t      = {:?}", sz);
    print!("\n  size of x      = {:?}", sx);
    print!("\n  size of y      = {:?}", sy);
    print!("\n  size of t + sz = {:?}", rt as i32 + sz as i32);
    putline();

    sub_title("mutating struct with unsafe pointer");
    unsafe {
        print!("\n  contents of ry = {:?}", *ry);
        print!("\n  mutating y through ry:");
        *ry = -3.2;
        print!("\n  contents of ry = {:?}", *ry);
    }
    t.show();
    putline();
}