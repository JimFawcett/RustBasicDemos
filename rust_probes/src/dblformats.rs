///////////////////////////////////////////////////////////
// dblformats.rs - printing floats with various formats  //
//                                                       //
// Jim Fawcett, https://JimFawcett.github.io             //
///////////////////////////////////////////////////////////

#[allow(unused_imports)]
use std::fmt::{self, Formatter, Display};
// https://doc.rust-lang.org/std/fmt/index.html
// https://cheats.rs/#formatting-strings

#[allow(dead_code)]
pub fn run() {
    
    print!("\n  -- float formats --\n");

    let d = 2.12345e+10;
    let f = 2.345e-5;
    // { [argument] ':' [[fill] align] [sign] ['#'] [width [$]] ['.' precision [$]] [type] }
    // format flags must appear in the order shown

    print!("\n  {:?}", d);
    print!("\n  {:?}", f);
    print!("\n  --{:>+#20.3e}--", d);
    print!("\n  --{:>+#20.3e}--", f);
    print!("\n  --{:<#20.7e}--", d);
    print!("\n  --{:<#20.7e}--", f);
    
    // format!(...) has syntax of println! but returns string instead of writing to console

    print!("\n  --{:^20}--", format!("{:e}", d));
}