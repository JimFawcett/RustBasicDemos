/////////////////////////////////////////////////////////////
// test_shortform_module::main.rs - demonstrate modules    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 20 Feb 2020  //
/////////////////////////////////////////////////////////////
// created with cargo new test_modules                     //
//   in RustBasicDemos::rust_modules                       //
/////////////////////////////////////////////////////////////

mod test1;
mod test2;

fn main() {
    print!("\n  Testing Modules");
    test1::hello();
    test2::hello();
    print!("\n\n");
}
