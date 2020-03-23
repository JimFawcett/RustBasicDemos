/////////////////////////////////////////////////////////////
// probe_traits.rs - demo user defined traits              //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 22 Mar 2020  //
/////////////////////////////////////////////////////////////

use std::fmt::{Debug};
use display::{*};
/*-----------------------------------------------
   Note that all trait functions are public
*/

/*-----------------------------------------------
   Show trait needs Super trait Debug 
   - provides default method implementation
*/
trait Show : Debug {
    fn show(&self) {
        print!("\n  {:?}", &self);
    }
}
/*-----------------------------------------------
   Stucts that use Size must implement fn size
   - no default impl provided
*/
trait Size {
    fn size(&self) -> usize;
}

#[derive(Debug, Copy, Clone)]
pub struct Test { // public type 
    x:i32, y:f64, // private data
}
/*-----------------------------------------------
   Implementing traits
*/
impl Show for Test {}  // using default impl
impl Size for Test {   // must provide impl
    fn size(&self) -> usize {
        use std::mem;
        mem::size_of::<Test>()
    }
}
/*-----------------------------------------------
   Implementing methods
*/
impl Test {
    pub fn new() -> Self {
        Self {
            x:42,
            y:1.5,
        }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn set_x(&mut self, v:i32) {
        self.x = v;
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn set_y(&mut self, v:f64) {
        self.y = v;
    }
}

#[allow(dead_code)]
pub fn run () {
    main_title("Demonstrating probe_traits");
    putline();

    sub_title("using Show trait");
    let mut t = Test::new();
    t.show();
    putline();

    sub_title("using getters and setters");
    t.set_x(3);
    t.set_y(-3.5);
    print!("\n  x = {}, y = {}", t.get_x(), t.get_y());
    putline();

    sub_title("using Size trait");
    print!("\n  size = Test instance t = {}", t.size());

    /*-------------------------------------------
       Implementing Size for built in types!
    */
    impl Size for i32 {
        fn size(&self) -> usize {
            std::mem::size_of::<i32>() as usize
        }
    }
    impl Size for f64 {
        fn size(&self) -> usize {
            std::mem::size_of::<f64>() as usize
        }
    }
    // let sx = std::mem::size_of::<i32>();
    // let sy = std::mem::size_of::<f64>();

    let sx = t.x.size();
    let sy = t.y.size();
    print!("\n  size of x = {}, size of y = {}", sx, sy);
    print!("\n  remaining 4 bytes is size of pointer to trait vtable");
    putline();

    sub_title("exploring struct layout with safe pointers");
    let mut t = Test::new();
    t.show();
    shows("\n  Note: Test implements traits:");
    shows("\n        Show, Size, Debug, Copy, Clone");
    shows("\n        Missing 4 bytes is ptr to traits vtable.\n");
    let rt = &t as *const Test;
    let rx = &t.x as *const i32;
    let ry = &mut t.y as *mut f64;
    let st = std::mem::size_of::<Test>();
    let sx = std::mem::size_of::<i32>();
    let sy = std::mem::size_of::<f64>();
    print!("\n  address of t   = {:?}", rt as i32);
    print!("\n  address of t.x = {:?}", rx as i32);
    print!("\n  address of t.y = {:?}", ry as i32);
    print!("\n  size of t      = {:?}", st);
    print!("\n  size of x      = {:?}", sx);
    print!("\n  size of y      = {:?}", sy);
    print!("\n  address of t + st = {:?}", rt as i32 + st as i32);
    putline();
}