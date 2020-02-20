/////////////////////////////////////////////////////////////
// module2::lib.rs - demonstrate module structue           //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 20 Feb 2020  //
/////////////////////////////////////////////////////////////
// created with cargo new module2 --lib                    //
//   in RustBasicDemos::rust_modules                       //
/////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub mod module2 {
    pub fn say() {
        print!("\n  hello from module2");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
