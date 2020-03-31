/////////////////////////////////////////////////////////////
// point_traits.rs - user defined traits for generic type  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 22 Mar 2020  //
/////////////////////////////////////////////////////////////

// http://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/

use std::fmt::{Debug};
use display::{*};
/*-----------------------------------------------
   Note that trait functions are always public
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
/*-----------------------------------------------
   Returns size of obj of any type w/ Size trait
   - uses type erasure via trait object
   - dyn holds two pointers
     - pointer to trait vtable
     - pointer to instance passed to fn
   - Uses dynamic dispatch, e.g., vtable lookup
     at run-time
*/
fn size_is(o:&dyn Size) ->usize {
  o.size()
}

#[derive(Debug, Copy, Clone)]
pub struct Point<T>{ // public type 
    x:T, y:T, z:T,   // private data
}
/*-----------------------------------------------
   Implementing traits
*/
impl<T> Show for Point<T> where T:Debug {}  
// using default impl
impl<T> Size for Point<T> {   // must provide impl
    fn size(&self) -> usize {
        std::mem::size_of::<Point<T>>()
    }
}
/*-----------------------------------------------
   Implementing methods
*/
impl<T> Point<T> {
    pub fn new(init_value: T) 
             -> Point<T> where T:Copy {
        Point {
            x: init_value,
            y: init_value,
            z: init_value,
        }
    }
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn set_x(&mut self, v:T) {
        self.x = v;
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
    pub fn set_y(&mut self, v:T) {
        self.y = v;
    }
    pub fn get_z(&self) -> &T {
        &self.z
    }
    pub fn set_z(&mut self, v:T) {
        self.z = v;
    }
}

#[allow(dead_code)]
pub fn run () {
    main_title("Demonstrating point_traits");
    putline();

    sub_title("using Show trait");
    let mut p = Point::new(0.0);
    p.show();
    putline();

    sub_title("using getters and setters");
    p.set_x(3.0);
    p.set_y(-3.5);
    p.set_z(1.0);
    print!(
        "\n  x = {}, y = {}, z = {}", 
        p.get_x(), p.get_y(), p.get_z()
    );
    p.show();
    putline();

    sub_title("using Size trait");
    print!(
        "\n  size of point instance p = {}", 
        p.size()
    );
    /*-------------------------------------------
       Implementing Size for built in types!!!
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

    let sx = p.x.size();
    let sy = p.y.size();
    let sz = p.z.size();
    print!(
        "\n  size of x = {}, size of y = {}, size of z = {}", 
        sx, sy, sz
    );
    putline();

    sub_title("exploring struct layout with safe pointers");
    let mut t = Point { x:0.0, y:1.0, z:0.5, };
    t.set_x(0.1);
    t.set_y(-0.1);
    t.show();
    shows("\n  Note: Point implements traits:");
    shows("\n        Show, Size, Debug, Copy, Clone\n");
    let rt = &t as *const Point<f64>;
    let rx = &t.x as *const f64;
    let ry = &t.y as *const f64;
    let rz = &t.z as *const f64;
    let st = size_is(&t);
    let sx = size_is(&t.x);
    let sy = size_is(&t.y);
    let sz = size_is(&t.z);
    print!("\n  address of t   = {:?}", rt as u32);
    print!("\n  address of t.x = {:?}", rx as u32);
    print!("\n  address of t.y = {:?}", ry as u32);
    print!("\n  address of t.z = {:?}", rz as u32);
    print!("\n  size of t      = {:?}", st);
    print!("\n  size of x      = {:?}", sx);
    print!("\n  size of y      = {:?}", sy);
    print!("\n  size of z      = {:?}", sz);
    print!(
        "\n  address of t + st = {:?}", 
        rt as u32 + st as u32
    );
    putline();
    print!("\n  size of pointer to t = {}", std::mem::size_of_val(&rt));
    type TraitType<'a> = &'a dyn Size;
    let tos = std::mem::size_of::<TraitType>();
    print!("\n  size of &dyn Size = {}", tos);
    putline();
    
    #[derive(Debug, Copy, Clone)]
    struct Test2 { x:i32, y:f64, };
    impl Show for Test2 {}
    impl Size for Test2 {   // must provide impl
        fn size(&self) -> usize {
            std::mem::size_of::<Test2>()
        }
    }
    let test2 = Test2 { x:0, y:0.0 };
    test2.show();
    print!("\n  size of Test2 = {}", size_is(&test2));
    print!("\n  size of Test2.x = {}", size_is(&test2.x));
    print!("\n  size of Test2.y = {}", size_is(&test2.y));
    print!(
        "\n  missing 4 bytes is struct padding for alignment"
    );
    putline();

    #[derive(Debug, Copy, Clone)]
    struct Test3 { x:i32, y:f64, z:i32, };
    impl Show for Test3 {}
    impl Size for Test3 {   // must provide impl
        fn size(&self) -> usize {
            std::mem::size_of::<Test3>()
        }
    }
    let test3 = Test3 { x:0, y:43.0, z:1 };
    test3.show();
    print!("\n  size of Test3 = {}", size_is(&test3));
    print!("\n  size of Test3.x = {}", size_is(&test3.x));
    print!("\n  size of Test3.y = {}", size_is(&test3.y));
    print!("\n  size of Test3.z = {}", size_is(&test3.z));
    putline();
}