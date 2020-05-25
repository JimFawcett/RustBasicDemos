/////////////////////////////////////////////////////////////
// function_probes - Demonstrates:                         //
// - pass by value and reference                           //
// - return by value and reference                         //
// - lifetime annotation                                   //
// - functions that accept other functions                 //
// Jim Fawcett, https://JimFawcett.github.io, 25 May 2020  //
/////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use display::*;
use std::fmt::Debug;

/////////////////////////////////////////////////
// Demo functions

/*-----------------------------------------------
   Accepts String by value
*/
fn show_str1(s:String) {
  print!("{}",&s);
}
/*-----------------------------------------------
   Accepts String by reference
*/
fn show_str2(s:&String) {
  print!("{}",&s);
}
/*-----------------------------------------------
   Accepts str by reference
*/
fn show_str3(s:&str) {
  print!("{}",&s);
}
/*-----------------------------------------------
   Returns String
*/
fn return_str1() -> String {
  let s = "test string 1".to_string();
  s  // moves s
}
/*-----------------------------------------------
   Returns string reference
   - it only makes sense to return a reference
     to a passed in ref or a static ref
   - compiler should prevent anything else
*/
fn return_str2() -> &'static str {
  let s = "test string 2";
  s
}
/*-----------------------------------------------
   Returns string reference
   - it only makes sense to return a reference
     to a passed in ref or a static ref
   - compiler should prevent anything else
*/
fn return_str3(s:&mut String) -> &mut String {
  s.push('Z');
  s
}
/*-----------------------------------------------
   Pass by value moves the argument into the
   function's stack frame, so invalid after call
*/
fn pass_by_value_str(s:String) {  // move
  show_type(&s);
  show_value(&s);
}
/*-----------------------------------------------
   Pass by ref borrows the argument. Borrow
   moved into the function's stack frame.  
   Borrow ends at end of function call, so 
   param is valid after call
*/
fn pass_by_ref_str(rs:&String) {  // borrow
  show_type(&rs);
  show_value(&rs);
}
/*-----------------------------------------------
   Pass by value moves the argument into the
   function's stack frame, so invalid after call
*/
fn pass_by_value<T>(t:T) where T:Debug {
  show_type(&t);
  show_value(&t);
}
/*-----------------------------------------------
   Pass by ref borrows the argument. Borrow
   moved into the function's stack frame.  
   Borrow ends at end of function call, so 
   param is valid after call
*/
fn pass_by_ref<T>(rt:&T) where T:Debug {
  show_type(&rt);
  show_value(&rt);
}
/*-----------------------------------------------
   Pass by ref borrows the argument into the
   function's stack frame.  Borrow ends at end
   of function stack frame, so param is valid 
   after call
*/
fn lifetime(rs:&String) -> String {
  show("\n  rs = ",rs); 
  show_type(rs);
  // replace doesn't attempt to mutate rs
  let s = rs.replace("z","a");
  show("\n  s = ",&s);  
  show_type(&s);
  shows("\n  returning string s by value (a move)");
  let put = |st|{ print!("{}", st); };
  put("\n  returning s by value"); 
  s  // return by value moves string to destination
}
// /*-----------------------------------------------
//    Pass by ref borrows the argument into the
//    function's stack frame.  Borrow ends at end
//    of function stack frame, so param is valid 
//    after call. Function returns a reference!
// */
fn lifetime2<'a, T>(rt:&'a T) -> & 'a T where T:Debug {
  // Lifetime annotation, 'a, enables borrow checker
  // to ensures that T lives at least as long as rt, 
  // its reference.
  show_type(&rt);
  show_value(&rt);
  rt  // the only time it makes sense to return a reference
      // is when returning a possibly modified input reference
}

/////////////////////////////////////////////////
// Useful functions

/*-----------------------------------------------
   Accepts either String or str
*/
fn shows<S: Into<String>>(s:S) {
  print!("{}",s.into());
}
/*---------------------------------------------------------
   Higher order function
*/
use std::panic;  // catch_unwind panic

fn tester(f:fn() -> bool, name:&str) -> bool {
  let rslt = panic::catch_unwind(|| { f() });  // simulate try-catch
  match rslt {
    Ok(true) => print!("\n  {} passed", name),
    Ok(false) => { print!("\n  {} failed", name); return false; },
    Err(_) => { print!("\n  {} paniced", name); return false; }
  }
  return true;
}

/* hide panic notification */
fn set_my_hook() {
  panic::set_hook(Box::new(|_|{ print!(" ");}));
}

fn always_fails() -> bool {
  false
}

fn always_succeeds() -> bool {
  true
}

#[allow(unreachable_code)]
fn always_panics() -> bool {
  panic!("always panics");
  return false;
}
/////////////////////////////////////////////////
// Test demo functions

fn main() {
    
  main_title("demo param passing");
   
  /*-------------------------------------------*/
  separator(48);
  sub_title("show_str1(s:String)");  // a move
  show_str1("\n  first test".to_string());

  /*-------------------------------------------*/
  separator(48);
  sub_title("show_str2(s:&String)");  // a borrow
  show_str2(&"\n  second test".to_string());

  /*-------------------------------------------*/
  separator(48);
  sub_title("show_str3(s:&str)");  // a borrow
  show_str3("\n  third test");

  /*-------------------------------------------*/
  separator(48);
  sub_title("return_str1() -> String");
  let mut s = return_str1();
  s.push_str(" with more stuff");
  let mut s1 = "\n  ".to_string();
  s1.push_str(&s);
  shows(s1);
  
  /*-------------------------------------------*/
  separator(48);
  sub_title("return_str2() -> &'static str");
  let s = return_str2();
  let mut st = s.to_string();
  st.push_str(" with more stuff");
  let mut s1 = "\n  ".to_string();
  s1.push_str(&st);
  shows(s1);
  
  /*-------------------------------------------*/
  separator(48);
  sub_title("return_str3(mut s:&mut String) -> &mut String");
  let mut s = String::from("test string 4");
  let rs = return_str3(&mut s);
  rs.push_str(" with more stuff");
  let mut s1 = "\n  ".to_string();
  s1.push_str(&rs);
  shows(s1);
  
  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass string by value");
  let mut s = "xyz".to_string();
  let s1 = s.clone();
  pass_by_value_str(s1);  // moves s1
  ////////////////////////////////////////////////
  // next statement fails to compile - s1 moved
  // pass_by_value(s1);

  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass string by reference");
  pass_by_ref_str(&s);
  s.push('a');
  pass_by_ref(&s);  // borrows s
  s.push('b');
  show("\n  after pushing a and b, s = ",&s);

  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass by value");
  let mut s = "xyz".to_string();
  let s1 = s.clone();
  pass_by_value(s1);
  ////////////////////////////////////////////////
  // next statement fails to compile - s1 moved
  // pass_by_value(s1);

  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass by reference");
  pass_by_ref(&s);
  s.push('a');
  pass_by_ref(&s);
  show("\n  after pushing a, s = ",&s);

  /*-------------------------------------------*/
  separator(48);
  sub_title("lifetime");
  let mut s = "xyz".to_string();
  show("\n  s = ",&s);
  shows("\n  calling lifetime");
  separator(30);
  let r = &mut lifetime(&mut s);
  separator(30);
  shows("\n  returned from lifetime");
  show("\n  s = ",&s);
  show("\n  r = ",&r);
  show_type(&r);
  r.push('b');
  show("\n  after pushing b, r = ",&r);
  s.push('b');
  show("\n  after pushing b, s = ",&s);

  /*-------------------------------------------*/
  separator(48);
  sub_title("lifetime2");
  let mut s = "xyz".to_string();
  show("\n  s = ",&s);
  let r = &mut lifetime2(&mut s);
  show_type(&r);
  show("\n  r = ",&r);

  /*-------------------------------------------*/
  separator(48);
  sub_title("shows<S: Into<String>>(s:S)");
  shows("\n  This accepts either String or str");

  /*-------------------------------------------*/
  separator(48);
  sub_title("Function pointer");
  let fun = pass_by_ref;
  let mut s = "xyz".to_string();
  fun(&s);
  s.push('a');
  fun(&s);

  /*-------------------------------------------*/
  separator(48);
  sub_title("lambdas");
  let l = |s:&str|{ show_type(&s); show_value(&s); };
  l("xyz");
  let s = String::from("abc");
  l(&s);

  /*-------------------------------------------*/
  separator(48);
  sub_title("higher_order_function");
  set_my_hook();

  let rstl = tester(always_fails, "always_fails");
  print!("\t\tresult = {}", rstl);
  let rstl = tester(always_succeeds, "always_passes");
  print!("\t\tresult = {}", rstl);
  let rstl = tester(always_panics, "always_panics");
  print!("\t\tresult = {}\n", rstl);

  /* show that we intercepted panic and continued */
  use std::io::{Write};
  let one_second = std::time::Duration::from_millis(1000);
  for i in 0..5 { 
    print!("\n  tick {}\t", 5 - i); 
    std::io::stdout().flush().unwrap();
    std::thread::sleep(one_second);
  };
  print!("\n\n  BOOM!\t");
  putlinen(2);
}
