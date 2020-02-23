#[allow(dead_code)]
fn show_copy() {
    print!("\n  copied");
}
#[derive(Debug)]
struct TestBlittable {
    pub id:u8,
}
impl Copy for TestBlittable { 
    // clone();
    // (|| print!("\n  copied"))();
    // // fn copy(&self) -> TestBlittable {
    // //     *self
    // // }
    //clone(&self)
}

impl Clone for TestBlittable {
    fn clone(&self) -> TestBlittable {
        print!("\n  cloned");
        *self
    }
}
impl TestBlittable {
    #[allow(dead_code)]
    pub fn new(id : u8) -> Self {
        Self {
            id,
        }
    }
}
fn main() {
   
    let x = TestBlittable::new(42);
    let mut y = x;
    println!("\n  y = {:?}\n", y);
    y = x.clone();
    // let cls = || print!("\n  a closure");
    // cls();
    // (|| print!("\n  another closure"))();
    println!("\n  y = {:?}\n", y);
}
