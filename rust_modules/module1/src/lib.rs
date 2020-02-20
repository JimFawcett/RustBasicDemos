/////////////////////////////////////////////////////////////
// module1::lib.rs - demonstrate module structue           //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 20 Feb 2020  //
/////////////////////////////////////////////////////////////
// created with cargo new module1 --lib                    //
//   in RustBasicDemos::rust_modules                       //
/////////////////////////////////////////////////////////////

#[allow(dead_code)]  // won't warn about unused
pub mod module1 {
    pub fn say() {
        print!("\n  hello from module1");
    }
}

/*-- unused test --*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
