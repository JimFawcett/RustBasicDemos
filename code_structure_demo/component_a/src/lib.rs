/*-----------------------------------------------
   code_structure_demo::component_a::ComponentA
   - implements trait TCompA
*/

// mod t_comp_a;
// use t_comp_a::{TCompA};
/*-----------------------------------------------
   trait TCompA will be used as an interface for
   ComponentA
*/
pub trait TCompA {
    fn do_work(&self);
    fn get_msg(&self) -> String;
    fn set_msg(&mut self, m:&str);
}
/*-----------------------------------------------
   Factory function
*/
#[allow(dead_code)]
pub fn get_instance() -> Box<dyn TCompA> {
        Box::new(ComponentA::new())  
}
/*-----------------------------------------------
   ComponentA - demonstrate interface and factory
*/
#[allow(dead_code)]
#[derive(Debug,Clone,PartialEq)]
pub struct ComponentA {
    name: String,
    msg: String,
}
/*-----------------------------------------------
   Implements interface trait TCompA
*/
impl TCompA for ComponentA {
    fn do_work(&self) {
        print!("\n  Component A working diligently");
        print!("\n  My messages is: {}", self.msg);
    }
    fn get_msg(&self) -> String {
        self.msg.clone()
    }
    fn set_msg(&mut self, m:&str) {
        self.msg = m.to_string();
    }
}
/*-----------------------------------------------
   Implement ComponentA methods
*/
#[allow(dead_code)]
impl ComponentA {
    pub fn new() -> Self {
        Self {
            name : "ComponentA".to_string(),
            msg : "no message".to_string(),
        }       
    }
}
// /*-----------------------------------------------
//    Factory function
// */
// #[allow(dead_code)]
// pub fn get_instance() -> Box<dyn TCompA> {
//     let a = ComponentA::new();
//     Box::new(a)
// }
/*-----------------------------------------------
   Unit tests
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let a = ComponentA::new();
        assert_eq!(a.name,"ComponentA".to_string());
        assert_eq!(a.msg,"no message".to_string());
    }
    #[test]
    fn test_set_msg() {
        let mut a = ComponentA::new();
        a.set_msg("a message");
        assert_eq!(a.msg,"a message".to_string());
    }
    #[test]
    fn test_get_msg() {
        let mut a = ComponentA::new();
        a.set_msg("a message");
        assert_eq!(a.get_msg(),"a message".to_string());
    }
    #[test]
    fn test_get_instance() {
        let a = ComponentA::new();
        let b = get_instance();
        assert_eq!(a.msg, b.get_msg());
    }
}
