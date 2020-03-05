/////////////////////////////////////////////////////////////
// demo_test::main.rs - library version of hello world     //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

mod helper;  // declares helper module
             // since there is no code block
             // helper.rs must be in local src and 
             // provide code for this module

use helper::{ say };

use demo_lib::{ libsay };

fn main() {
    helper::say();
    say();
    demo_lib::libsay();
    libsay();
    print!("\n  Hello, world!\n\n");
}
