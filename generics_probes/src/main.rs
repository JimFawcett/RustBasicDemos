use std::fmt::{*};
use display::{*};

/*-----------------------------------------
   No copy arguments will be moved
*/
fn demo<T:Debug>(t:T) {
    show_type(&t);
    show_value(&t);
}
  
/*-----------------------------------------
   refs so arguments will not be moved 
*/
fn demo_ref<T>(t:&T) where T:Debug  {
    show_type(t);
    show_value(t);
}

#[derive(Debug)]
struct Point<T> { x:T, y:T, z:T, }

// this works because blittable, provided that T is blittable
#[derive(Debug, Copy, Clone)]
struct BetterPoint<T> { x:T, y:T, z:T, }

/* test */

  fn main() {

    main_title("generics_probes");
    let mut s = String::from("this is a test");
    
    sub_title("demo_ref");
    demo_ref(&s);
    let pi = 3.1415927;
    demo_ref(&pi);
    s.push('Z');
    putline();

    sub_title("demo");
    demo(s);
    demo(pi);
    // statement below won't compile - s moved
    // s.push('Z');
    putline();

    sub_title("demo_ref and demo with struct");
    let mut pt = Point { x:1.0, y:-1.5, z:2.3 };
    demo_ref(&pt);
    pt.x = 3.2;
    demo(pt);
    // statement below won't compile - pt moved for demo
    // pt.x = 3.2;
    putline();

    sub_title("demo_ref and demo with copy-able struct");
    let mut bpt = BetterPoint { x:1.0, y:-1.5, z:2.3 };
    demo_ref(&bpt);
    bpt.x = 3.2;
    demo(bpt);
    // statement ok - pt copied for demo
    bpt.x = 3.2;
    putline();

    sub_title("demo_ref and demo with copy-able struct");
    let mut bpt = BetterPoint { x:"one", y:"two", z:"three" };
    demo_ref(&bpt);
    bpt.x = "1";
    demo(bpt);
    // statement ok - pt copied for demo
    bpt.x = "one";
    putline();

    sub_title("demo_ref and demo with non copy-able struct");
    let mut bpt = BetterPoint { 
        x:"one".to_string(), 
        y:"two".to_string(), 
        z:"three".to_string() 
    };
    demo_ref(&bpt);
    bpt.x = "four".to_string();
    demo(bpt);
    // statement won't compile - pt not blittable
    // bpt.x = "one".to_string();
     
    putlinen(2);
}
