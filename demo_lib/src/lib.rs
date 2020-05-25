/////////////////////////////////////////////////////////////
// demo_lib::lib.rs - library package for demo             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////
/*
   Demonstrate with cargo test -q command
*/
pub fn libsay() {
    print!("\n  hello from demo_lib");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        libsay();
        assert!(true);
    }
}
