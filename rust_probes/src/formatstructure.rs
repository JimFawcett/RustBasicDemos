/////////////////////////////////////////////////////////////
// formatstructure.rs - demo print formats                 //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io               //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use std::fmt::{self, Formatter, Display};

// https://doc.rust-lang.org/std/fmt/index.html
// https://cheats.rs/#formatting-strings

#[allow(dead_code)]
pub fn run() {
    
    print!("\n  -- format structure --\n");

    print!("\n  fixed stuff here");

    let demo = "variable stuff here";
    print!("\n  {}", demo);

    let jim = "Jim";
    let job = "Retired";
    print!("\n  {} is {}", jim, job);

    let one = stringify!(one);
    let two = stringify!(two);
    print!("\n  {0}, {1}, {0}", one, two);

    print!("\n  {jack}, {jill}, {helina}", 
      jack=stringify!(Jack), 
      jill=stringify!(Jill), 
      helina=stringify!(Helina)
    );

    let demo = 27;
    print!(
      "\n  {0} as decimal : {0:?}\n  {0} as hex :     {0:x}\n  {0} as binary :  {0:b}"
      , demo
    );
}