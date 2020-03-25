// demo.rs - demo ComponentA

extern crate component_a;
// mod t_comp_a;
// use t_comp_a::{TCompA};
use component_a::{*};
use display::{*};

fn main() {
  main_title("Demonstrating ComponentA");

  let mut a = ComponentA::new();
  a.set_msg("msg from a");
  a.do_work();
  let mut a2 = get_instance();
  a2.set_msg("msg from a2");
  print!("\n  {}", a2.get_msg());
  putline();
}