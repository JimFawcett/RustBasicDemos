/////////////////////////////////////////////////////////////
// ownership.rs - demonstrates Rusts ownership rules       //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

use std::fmt::{Debug};

/*-- will use to show borrow of argument --*/
#[allow(dead_code)]
pub fn show<T:Debug>(s:String, t:T) {
    print!("\n  {}{:?}", s, t);
}

#[allow(dead_code)]
pub fn run() {
  /*-------------------------------------------------*/
    print!("\n  -- demo blittable ownership --\n");
  /*---------------------------------------------------
       Blitables are values that can be copied by copying
       their bits.
       - primitive types
       - arrays of blittable are blittable
       - tuples and structs with blittable fields
       - literal strings
       They have no resources stored on heap, so no cleanup
       when they go out of scope.
    */
    let i = 3;        // i is blitable
    // i=4; compile error, i is not mutable

    let mut j = i;    // j gets copy of i
    print!("\n  i = {}, j = {}", i, j);

    let r = &mut j;   // r borrows j, e.g., reference, no copy
    *r += 2;          // j is mutated
    print!("\n  i = {}, j = {}", i, j);
    j += 2;
    print!("\n  i = {}, j = {}", i, j);

    let arr = [1, 2, 3];
    print!("\n  arr = {:?}", arr);
    let mut new_arr = arr;
    new_arr[2] = -1;
    print!("\n  new_arr = {:?} - mutated", new_arr);
    
    // new_arr[3] = 0;  // this is a COMPILE error, not run-time error
    // Rust checks bounds at compile-time, so no run-time cost

    let t = (1, 1.5, 'z');
    print!("\n  tuple t = {:?}", t);
    let mut u = t;
    u.0 = 2;
    print!("\n  tuple u = {:?} - mutated", u);
    print!("\n  tuple t = {:?}", t);

    let u = (1, 1.5, "a string");  // all blittable
    print!("\n  u = {:?}", u);
    let v = u;
    print!("\n  v = {:?}", v);
    print!("\n  u = {:?}", u);

    /*-- function show borrows u --*/
    show("\n  borrowed u = ".to_string(), &u);
    print!("\n");

    /*-- blittable string literals --*/

    let s:&str = "a literal string";
    let t = s;  // t gets copy of s
    print!("\n  s = {:?}", s);
    print!("\n  t = {:?}", t);

    /*-- blittable structs --*/
    #[derive(Debug, Copy, Clone)]   // Copy, Clone work because S1 blittable
    struct S1 { x:i32, y:f64, z:char, };
    let s:S1 = S1{ x:1, y:1.5, z:'Z' };
    print!("\n  s = {:?}", s);
    let mut t = s; 
    print!("\n  t = {:?}", t);
    t.y = -2.0;
    print!("\n  t = {:?} - mutated", t);
    print!("\n  s = {:?} - copied", s); 
    let r = &mut t;
    r.z = 'a';
    print!("\n  r = {:?} - ref of t mutated", r);
    print!("\n  t = {:?} - t is mutated too", t);

  /*-----------------------------------------------------*/
    print!("\n\n  -- demo non-blittable ownership --\n");
  /*-------------------------------------------------------
       Non-blittable types can't be copied:
       - let x = String::from("a string");
       - let y = x;   // x moved to y, not copied
       When y goes out of scope the String::Drop function
       is called to free the string's heap memory.
       - Drop is a trait (we'll discuss that later)
    */
    /*-- strings --*/
    let s = String::from("String named s");  // s is not blitable
    let mut t = s;                           // s is moved to t
    t.push_str(" with more stuff");
    print!("\n  t = {}", t);
    // print!("\n  s = {}", s);              // can't use s, been moved

    /*-- string borrowing --*/
    t = "a new string value".to_string();    // original contents dropped
    print!("\n  t = {}", t);
    let rt = &mut t;
    // compile fails - 2nd mutable ref
    //   let rrt = &mut t;
    // compile fails - already have &mut t
    //    let rrt = &t;
    rt.push_str(" with more stuff");  // ok
    // compile fails - owner mutated while borrow is active
    //   t.push_str(" and still more stuff");
    //   rt.push_str(" and still more");
    // ok - owner mutated while borrow is active but borrow not used
    t.push_str(" and still more stuff");
    // uncomment the next line and previous line will fail to compile
    //   rt.push_str(" and still more");
    t.push_str(" and again more stuff");
    print!("\n  t = {}", t);
    drop(t);
    // statement below fails to compile, t dropped
    // t.push('s');

    /*-- tuple with string member --*/
    let x = (1, 1.5, "a string".to_string());  // x.2 is not blittable
    print!("\n  x = {:?}", x);
    let y = x;
    print!("\n  y = {:?}", y);
    // print!("\n  x = {:?}", x);       // won't compile, x was moved

    /*-- non-blittable structs --*/
    #[derive(Debug)]   // can't implement Copy trait, not blittable
    struct S { x:i32, y:f64, z:String, };
    let s: S = S{ x:1, y:1.5, z:"a string".to_string() };
    print!("\n  s = {:?}", s);
    let mut t = s; 
    print!("\n  t = {:?}", t);
    t.y = -2.0;
    print!("\n  t = {:?} - mutated", t);
    // print!("\n  s = {:?} - copied", s);  // won't compile, value moved to t
    let r = &mut t;
    r.z = String::from("another string");
    print!("\n  r = {:?} - ref of t mutated", r);
    print!("\n  t = {:?} - t is mutated too", t);

    let q = &t;                   // makes r invalid
    print!("\n  q = {:?}", q);
    //print!("\n  r = {:?}", r);  // compile error
    t.y = 0.25;                   // makes q invalid
    print!("\n  t = {:?}", t);
    //print!("\n  q = {:?}", q);  // compile error

    /*-----------------------------------------------------
       A rust program may have any number of immutable
       references to some value if there are no mutable
       references to that value.

       It may only have one mutable reference to some value
       and then no immutable references are allowed.

       Compile errors occur on use, not declaration.
    */
    /*-- values on heap --*/
    let s:Box<f64> = Box::new(2.5);
    print!("\n  s:Box<f64> = {:?}", s);
    let t = s;
    print!("\n  t:Box<f64> = {:?}", t);
    // print!("\n  s:Box<f64> = {:?}", s); // won't compile, s moved
    println!();

    let s = String::from("will be dropped");
    print!("\n  {:?}", s);
    drop(s);
    ///////////////////////////////////////////////////////////
    // statement below fails to compile because s was dropped
    //print!("\n  trying to display s again: {:?}", s);   
  
    /*-----------------------------------------------------*/
    print!("\n\n  -- demo dropping reference restores owners ability to mutate --\n");
    let mut x = 2;
    let r = &mut x;
    *r = 3;
    //---------------------------------
    // state below fails to compile
    // x = 4;  // owner can't mutate borrowed value

    print!("\n  r = {}", r);
    drop(r);
    x += 2;  // x's ability to mutate restored
    print!("\n  x = {}", x);

    print!("\n\n  That's all Folks!\n");
}