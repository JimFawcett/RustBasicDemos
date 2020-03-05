/////////////////////////////////////////////////////////////
// demo_test::helper.rs - module for demo                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

pub fn say() {
    print!("\n  hello from helper");
}

// Including a main function is not a good idea
// but doesn't seem to do any harm, even if pub.
#[allow(dead_code)]
fn main() {
    print!("\n  hello world");
}