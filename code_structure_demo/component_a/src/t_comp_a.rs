/*-----------------------------------------------
   trait TCompA will be used as an interface for
   ComponentA
*/
pub trait TCompA {
    fn do_work(&self);
    fn get_msg(&self) -> String;
    fn set_msg(&mut self, m:&str);
}

#[allow(dead_code)]
#[allow(improper_ctypes)]
extern "C" { fn get_instance() -> Box<dyn TCompA>; }
