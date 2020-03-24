// demo.rs - demo ComponentA

extern crate component_b;

use component_a::{get_instance};
use component_b::{*};
use display::{*};

fn main() {
  main_title("Demonstrating ComponentA");

  let mut b = ComponentB::new();
  b.set_msg("msg from a");
  let mut b2 = get_instance();
  b2.set_msg("msg from b2");
  print!("\n  {}", b2.get_msg());
  putline();
}