/////////////////////////////////////////////////////////////
// demodrop.rs - demo drops                                //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io               //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use std::fmt::{self, Formatter, Display};

// https://doc.rust-lang.org/std/fmt/index.html
// https://cheats.rs/#formatting-strings

///////////////////////////////////////////////////////////
// DemoCopy is copyable and clonable struct
//---------------------------------------------------------
// DemoCopy's field, a float, is copyable, 
// so DemoCopy is copyable
//---------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct DemoCopy {
    pub f:f64,  
}
impl DemoCopy {
    #[allow(dead_code)]
    pub fn new(f : f64) -> Self {
        Self {
            f,
        }
    }
}
///////////////////////////////////////////////////////////
// DemoDrop implements Drop trait
//---------------------------------------------------------
// Language prevents impl Copy if Drop is impl
// DemoDrop's field, a string, is not copyable
//---------------------------------------------------------
#[derive(Debug)]
pub struct DemoDrop {
    pub s:String,
}
// can't define Copy trait if Drop trait is defined
// or if fields are not blitable
impl Drop for DemoDrop { 
    fn drop(&mut self) {
        print!("\n  dropping DemoDrop");
    }
}
impl DemoDrop {
    #[allow(dead_code)]
    pub fn new(s : String) -> Self {
        Self {
            s,
        }  // note: no semicolon so Self is returned
    }
}
///////////////////////////////////////////////////////////
// DemoClone implements Clone trait
//---------------------------------------------------------
// can't define Copy trait because string field is not blitable
// but String defines clone, so we can implement clone.
//---------------------------------------------------------
#[derive(Debug)]
pub struct DemoClone {
    pub s:String,
}
impl Clone for DemoClone {  // can't define Copy since String is not blitable
    fn clone(&self) -> Self {
        DemoClone::new(self.s.clone()) // note: no semicolon so will be returned
        // This works too
        // DemoClone { s : self.s.clone(), }
    }
}
impl DemoClone {
    #[allow(dead_code)]
    pub fn new(s : String) -> Self {
        Self {
            s,
        }
    }
}
/*-- Demonstrate copies, moves, clones, and drops --*/

#[allow(dead_code)]
pub fn run() {
    
    print!("\n  -- demo copies, moves, clones, and drops --\n");


    let dint1 : i32 = 3;
    print!("\n  dint1 = {:?} - original", dint1);

    // ints are copyable so copy here
    let dint2 : i32 = dint1;
    print!("\n  dint2 = {:?} - copy", dint2);
    // so dint1 still valid
    print!("\n  dint1 = {:?} - original, still valid\n", dint1);

    let s1 : String = "string for demo move".to_string();
    print!("\n  s1 = {:?} - original", s1);
 
    let s2 = s1;
    print!("\n  s2 = {:?} - a move, original invalid", s2);
    
    // fails to compile as s1 was source of move so invalid
    //print!("\n  s1 = {:?} - original", s1);

    let s3 = s2.clone();
    print!("\n  s3 = {:?} - a clone", s3);
    print!("\n  s2 = {:?} - original, s2, still valid\n", s2);

    /*-- demo string references --*/

    ///////////////////////////////////////////////////////
    // compiles with some rule violations.  Compile fails
    // when we attempt to use the illegal references.
    // The rules are:
    // - may have any number of active immutable references
    //   provided there are no mutable references
    // - may only have one mutable reference
    //-----------------------------------------------------
    let mut s1 = String::from("s1\'s value");
    print!("\n  s1 = \"{}\" original", s1);
    let s1r = &mut s1;
    print!("\n  s1r = &mut s1 = \"{}\" - ref to original", s1r);
    s1r.push_str(" more stuff");
    print!("\n  s1r = \"{}\" - mutated value", s1r);
    print!("\n  s1 = \"{}\" - mutated but still valid", s1);
    let s2r = &s1;
    print!("\n  s2r = &s1 = \"{}\" - ref value", s2r);
    let s3r = &mut s1;
    print!("\n  s3r = &mut s1 = \"{}\" - ref value", s3r);
    print!("\n  s1 = \"{}\" - mutated but still valid", s1);

    ///////////////////////////////////////////////////////
    // Fails to compile because there is another reference 
    // to s1
    //----------------------------------------------------
    // s3r.push_str(" - try illegal move");
    // print!("\n  s3r = {:?} - after illegal move",s3r);
    
    print!("\n");

    /*-- demo struct copy --*/

    let dd1 :DemoCopy = DemoCopy::new(1.2345);
    print!("\n  dd1 = {:?} - original", dd1);

    let dd2 = dd1;
    print!("\n  dd2 = {:?} - a copy", dd2);

    let dd3 = dd1;
    print!("\n  dd3 = {:?} - a copy", dd3);

    let dd4 = dd1.clone();
    print!("\n  dd4 = {:?} - a clone", dd4);

    print!("\n  dd1 = {:?} - original still valid\n", dd1);

    /*-- demo struct move --*/

    let dd1 :DemoDrop = DemoDrop::new("another string".to_string());
    print!("\n  dd1 = {:?} - original", dd1);

    // structs with string fields are not copyable so move here
    let dd2 = dd1;
    print!("\n  dd2 = {:?} - a move, original invalid\n", dd2);

    // This fails to compile because dd1 was moved from
    // let dd3 = dd1;
    // print!("\n  dd3 = {:?}", dd3);

    /*-- demo struct clone --*/

    let dd1 :DemoClone = DemoClone::new("another string".to_string());
    print!("\n  dd1 = {:?} - original", dd1);

    // using clone trait impl so no move here 
    let dd2 = dd1.clone();
    print!("\n  dd2 = {:?} - a clone", dd2);

    // since dd1 was not moved from it is still valid
    print!("\n  dd1 = {:?} - original still valid", dd1);
}