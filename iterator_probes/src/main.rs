/////////////////////////////////////////////////////////////
// iterator_probes - iterate over various collections      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 25 May 2020  //
/////////////////////////////////////////////////////////////

use display::{*};

#[allow(dead_code)]
fn show_prefix(p:&str) {
    print!("{}", p);
}
#[allow(dead_code)]
/*-- note: Iterator has an associated type, Item --*/
fn show_csl<T>(mut t:T) where T:Iterator, T::Item:std::fmt::Debug, {
    print!("{:?}", t.next().unwrap());  // no leading comma
    for val in t {
        print!(", {:?}", val);          // leading comma
    }
}
#[allow(dead_code)]
/*-- note: Iterator has an associated type, Item --*/
fn display_csl<T>(mut t:T) where T:Iterator, T::Item:std::fmt::Display, {
    print!("{}", t.next().unwrap());  // no leading comma
    for val in t {
        print!(", {}", val);          // leading comma
    }
}
#[allow(dead_code)]
fn show_pcsl<T>(t:T) where T:Iterator, T::Item:std::fmt::Debug,  {
    show_prefix("\n  ");
    show_csl(t);
}
#[allow(dead_code)]
fn display_pcsl<T>(t:T) where T:Iterator, T::Item:std::fmt::Display,  {
    show_prefix("\n  ");
    display_csl(t);
}
fn main() {

    main_title("Demonstrating Iterators");
    putline();
    
    shows("\n-- demo iter over vec of ints --");
    let v = vec![1, -1, 2, -2, 3, -3];
    print!("\n  ");
    let it = v.iter();
    for val in it {
        print!("{} ", val);
    }
    shows("\n-- demo comma separated display --");
    show_pcsl(v.iter());
    putline();

    shows("\n-- demo iter over instance of map type --");
    show_prefix("\n  ");
    use std::collections::HashMap;
    let mut map:HashMap<&str, i32> = HashMap::new();
    map.entry("zero").or_insert(0);
    map.entry("one").or_insert(1);
    map.entry("two").or_insert(2);
    map.entry("three").or_insert(3);
    let it_map = map.iter();
    for val in it_map {
        print!("{:?} ", val);
    }
    putline();

    shows("\n-- demo iter over array of floats --");
    let a = [1.0, 2.2, 3.3, 4.4, 5.5];
    print!("\n  ");
    let ita = a.iter();
    for val in ita {
        print!("{} ", val);
    }
    shows("\n-- demo comma separated display --");
    let iter = a.iter();
    show_pcsl(iter);
    putline();

    shows("\n-- demo iter over array of strs --");
    let mut s = [ "one", "two", "three", "four" ];
    show_pcsl(s.iter());
    shows("\n-- demo using Display trait instead of Debug --");
    display_pcsl(s.iter());
    putline();

    shows("\n-- demo map function modifying items --");
    show_prefix("\n  ");
    
    let iter = s.iter_mut().map(|item| { 
        let mut mod_item:String = item.to_string();
        mod_item.push('z');
        mod_item   // returning String, not str - this works
    });

    for val in iter {
        print!("{:?} ",val);
    }
    putline();

    shows("\n-- demo collect items into Vec --");
    let iter = s.iter_mut().map(|item| { 
        let mut mod_item:String = item.to_string();
        mod_item.push('z');
        mod_item   // returning String, not str - this works
    });
    let v:Vec::<String> = iter.collect();
    print!("\n  {:?}", v);
    putline();

    println!("\n  That's all Folks!\n");
}
