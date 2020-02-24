/////////////////////////////////////////////////////////////
// demo_lib::lib.rs - library package for demo             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

pub fn libsay() {
    print!("\n  hello from demo_lib");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
