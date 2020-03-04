// function_probes

#[allow(unused_imports)]
use display::*;
use std::fmt::Debug;

/*-----------------------------------------------
   Pass by value moves the argument into the
   function's stack frame, so invalid after call
*/
fn pass_by_value<T>(t:T) where T:Debug {
  show_type(&t);
  show_value(&t);
}

/*-----------------------------------------------
   Pass by ref borrows the argument into the
   function's stack frame.  Borrow ends at end
   of function stack frame, so param is valid 
   after call
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
  let rs = rs.replace("z","a");
  show_type(&rs);
  show_value(&rs);  
  rs
}

// /*-----------------------------------------------
//    Pass by ref borrows the argument into the
//    function's stack frame.  Borrow ends at end
//    of function stack frame, so param is valid 
//    after call
// */
fn lifetime2<'a, T>(rt:&'a T) -> & 'a T where T:Debug {
  show_type(&rt);
  show_value(&rt);
  rt
}

trait Test {
  fn testing<T:Debug>(t:T) -> bool;
}

fn main() {
    
  main_title("demo param passing");
    
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

    /*-------------------------------------------*/
    separator(48);
    sub_title("lifetime");
    let mut s = "xyz".to_string();
    show("s = ",&s);
    let r = &mut lifetime(&mut s);
    show_type(&r);
    show("r = ",&r);
    r.push('b');
    show("r = ",&r);
    s.push('b');
    show("s = ",&s);

    /*-------------------------------------------*/
    separator(48);
    sub_title("lifetime2");
    let mut s = "xyz".to_string();
    show("s = ",&s);
    let r = &mut lifetime2(&mut s);
    show_type(&r);
    show("r = ",&r);

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

    // //let t = Test;
    // show_type(&_:Test);
    // putline();
}
