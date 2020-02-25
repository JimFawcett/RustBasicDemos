/////////////////////////////////////////////////////////////
// vector_probes::main.rs - basic vector operations        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 Feb 2020  //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use display::{ log, show, putline, putlinen, main_title, sub_title };
#[allow(unused_imports)]
use std::fmt::{ Debug, Display };

fn main() {

    main_title("vector_probes");
    putline();

    /*-- create vector and mutate --*/
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    log(&v);

    {
        /*-- borrow element of v --*/
        let elem = &mut v[1];
        *elem = -2;
        log(elem);

        /////////////////////////////////////////////
        // fails to compile - second mutable borrow
        // v.push(4);
        // log(&v);
        // log(elem);  // without this, will compile

    }   // elem borrow ends here

    v.push(4);
    log(&v);
    putline();

    let v = vec![5, 4, 3, 2, 1];  // original v dropped
    show("v = ", &v);
    
    let vc = v.clone();
    show("v clone() = ", &vc);

    let vs = &v[1..3];
    show("v slice = &v[1..3] = ", &vs);

    /////////////////////////////////////////////////
    // Equivalent:
    //-----------------------------------------------
    // let s = String::from("v slice = &v[1..3] = ");
    // show(&s, &vs);
    
    /////////////////////////////////////////////////
    // Demo index out of bounds:
    //-----------------------------------------------
    // std::env::set_var("RUST_BACKTRACE", "1");
    // log(&v[5]);  // panic - out of bounds index
   
    putline();
    sub_title("That's all Folks!");
    putlinen(2);
}
