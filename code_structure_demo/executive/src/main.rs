/*-----------------------------------------------
   code_structure_demo::executive::Executive
   - uses component_a and component_b
*/

use component_a::{get_instance};
use component_b::{*};
use display::{*};

pub struct Executive {
  b:ComponentB,
}
impl Executive {
  fn new() -> Self {
    Executive { b:ComponentB::new(), }
  }
  fn run(&mut self) {
    /*-- use ComponentB --*/
    self.b.set_msg("msg from ComponentB");
    let msg = self.b.get_msg();
    print!("\n  {}",msg);
    /*-- use ComponentA through ComponentB --*/
    let mut a = self.b.get_comp_a();
    a.set_msg("msg from ComponentA");
    print!("\n  {}", a.get_msg());
    /*-- use ComponentA directly through factory --*/
    let mut a2 = get_instance();
    a2.set_msg("msg from Executive's instance of ComponentA");
    print!("\n  {}", a2.get_msg());
    putline();
  }
}

fn main() {
  main_title("Demonstrating Executive");

  let mut exec = Executive::new();
  exec.run();
  putline();
}
