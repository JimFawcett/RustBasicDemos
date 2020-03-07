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
  rs  // return by value moves string to destination
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
  rt  // the only time it makes sense to return a reference
      // is when returning a possibly modified input reference
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
