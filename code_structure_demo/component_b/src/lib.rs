/*-----------------------------------------------
   code_structure::component_b::ComponentB
   - aggregates instance of ComponentA as TCompA
*/
use component_a::{ TCompA, get_instance };

/*-----------------------------------------------
   ComponentB
*/
#[allow(dead_code)]
pub struct ComponentB {
    name: String,
    msg: String,
}
/*-----------------------------------------------
   Implement ComponentB methods
*/
#[allow(dead_code)]
impl ComponentB {
    pub fn new() -> Self {
        Self {
            name : "ComponentB".to_string(),
            msg : "no message".to_string(),
        }
    }
    pub fn set_msg(&mut self, m:&str) {
        self.msg = m.to_string();
    }
    pub fn get_comp_a(&self) -> Box<dyn TCompA> {
        get_instance()
    }
}
/*-----------------------------------------------
   ComponentB unit tests
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let b = ComponentB::new();
        assert_eq!(b.name,"ComponentB".to_string());
        assert_eq!(b.msg,"no message".to_string());
    }
    #[test]
    fn test_set_msg() {
        let mut b = ComponentB::new();
        b.set_msg("a message");
        assert_eq!(b.msg,"a message".to_string());
    }
    #[test]
    fn test_get_a() {
        let b = ComponentB::new();
        let my_a = b.get_comp_a();
        let inst_a = get_instance();
        assert_eq!(my_a.get_msg(), inst_a.get_msg());
    }
}
