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

use module1::module1::{say as say1};
use module2::module2::{say as say2};

// extern crate module1;  /*- depricated import -*/
// extern crate module2;

fn main() {
    say1();
    say2();
    mod1::say();
    mod2::say();
    module1::module1::say();  // does'nt need any imports
    module2::module2::say();
    print!("\n  Hello from test_modules\n\n");
}
