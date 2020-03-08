use display::{*};

/*-----------------------------------------------
  Function consume accepts predicate closure
  - executes closure and returns its value
*/
fn consume<F: FnOnce() -> bool>(cl:F) -> bool 
where F: FnOnce() -> bool {
  cl()
}
/*-----------------------------------------------
  Function answer accepts bool and displays value
*/
fn answer(ans:bool) {
    if ans == true {
        print!("\n  answer is true");
    }
    else {
        print!("\n  answer is false");
    }
}
/*-----------------------------------------------
  pure predicate functions
*/
fn always_true() -> bool { true }
fn always_false() -> bool { false }

/*-----------------------------------------------
  Demonstrate executor and closures
*/
fn main() {
    main_title(" -- demo closures -- ");

    let x:i32 = 3;  // immutable capture, below
    let cl = |val:i32| { 
      print!("\n  val = {}, x = {}, val + x = {}", val, x, val + x) 
    };
    cl(7);
    let put = putline;  // declaring funptr
    put();

    let mut count = 0;
    let mut counter = |offset:i32| {  // mutable closure
        count = count + 1;
        print!("\n  count = {}, sum = {}", 
          count, 
          count + offset
        )
    };
    counter(2);
    counter(1);
    counter(0);
    putline();

    let clst = ||{ true };  // invariant closure
    let clsf = ||{ false }; // invariant closure

    let mut ans = consume(clst);
    answer(ans);
    ans = consume(clsf);
    answer(ans);
    ans = consume(always_true);
    answer(ans);
    ans = consume(always_false);
    answer(ans);
    putlinen(2); 
}
