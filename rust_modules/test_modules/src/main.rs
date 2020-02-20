/////////////////////////////////////////////////////////////
// test_modules::main.rs - demonstrate module structue     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 20 Feb 2020  //
/////////////////////////////////////////////////////////////
// created with cargo new test_modules                     //
//   in RustBasicDemos::rust_modules                       //
/////////////////////////////////////////////////////////////

/*-- imports --*/
use module1::module1 as mod1;  // use crate::module as mod
use module2::module2 as mod2;  // cargo expects these to be libs

// extern crate module1;  /*- depricated import -*/
// extern crate module2;

fn main() {
    mod1::say();  //module1::module1::say();
    mod2::say();
    print!("\n  Hello from test_modules\n\n");
}
