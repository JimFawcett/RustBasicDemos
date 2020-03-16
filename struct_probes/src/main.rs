// struct_probes::main.rs

#[allow(unused_imports)]
use display::{*};
use std::fmt;

/*-- basic demo --*/
#[derive(Debug)]
struct Person1 {
    name:String, occup:String, id:u32,
}
#[allow(dead_code)]
impl Person1 {
    fn show(&self) {
        print!("\n  Person1: {:?}", &self);
    }
}
#[derive(Debug)]
struct Person2 (
    String, String, u32
);
#[allow(dead_code)]
impl Person2 {
    fn show(&self) {
        print!("\n  Person2: {:?}", &self);
    }
}
#[derive(Debug)]
struct Person3;
#[allow(dead_code)]
impl Person3 {
    fn show(&self) {
        print!("\n  Person3");
    }
}

/*-- enums used for Demo1 and Demo2 --*/
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Level { Basic, Intermediate, Advanced }
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Topic { Rust, Cpp, Design, }

/*-- Struct Demo1 has private data --*/
#[derive(Debug)]
pub struct Demo1 {
    name: String,
    level: Level,
    topic: Topic,
}
/*-- implement methods for Demo1 --*/
#[allow(dead_code)]
impl Demo1 {
    pub fn new() -> Self {  // set default values
        Self {
            name:String::from(""),
            level: Level::Basic,
            topic: Topic::Rust,
        }
    }
    pub fn set_name(&mut self, s:&str) { 
        self.name = s.to_string(); 
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_level(&mut self, l:Level) {
        self.level = l;
    }
    pub fn get_level(&self) -> &Level {
        &self.level
    }
    pub fn set_topic(&mut self, t:Topic) {
        self.topic = t;
    }
    pub fn get_topic(&self) -> &Topic {
        &self.topic
    }
}

/*-- struct Demo2 has public data --*/
#[derive(Debug)]
pub struct Demo2 {
    pub name: String,
    pub level: Level,
    pub topic: Topic,
}
/*-- implement Display trait for Demo2 --*/
#[allow(dead_code)]
impl fmt::Display for Demo2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
          f, 
          "Demo2 {{ name: \"{}\", level: {:?}, topic: {:?} }}", 
          self.name, self.level, self.topic)
    }
}
/*-- implement Default trait for Demo2 --*/
impl Default for Demo2 {
    fn default() -> Self {
        Self {
            name: String::from(""),
            level: Level::Basic,
            topic: Topic::Rust,
        }
    }
}
/*-- implement init method for Demo2 --*/
impl Demo2 {
    pub fn init(self) -> Demo2 {
      Demo2 {
        name: self.name,
        level: self.level,
        topic: self.topic,
      }
    }
}

fn main() {

    sub_title("Demonstrating Basic Structs");
    let p1 = Person1 { 
        name:"Jim".to_string(), 
        occup:"dev".to_string(), 
        id:42 
    };
    p1.show();
    let p2 = Person2 { 
        0:"Jim".to_string(),
        1:"dev".to_string(),
        2:42
    };
    p2.show();
    let p3 = Person3;
    p3.show();
    putline();
    
    sub_title("Demonstrating Demo1 Struct");
    let mut demo1 = Demo1::new();
    demo1.set_name("Demo1 probe");
    print!("\n  {:?}", demo1);
    print!("\n  Demo1 level = {:?}", demo1.get_level());
    putline();

    sub_title("Demonstrating Demo2 Struct");
    let mut demo2 = Demo2 {
        name: String::from("Jim's Demo2"),
        ..Default::default()
    }.init();

    print!("\n  Using Debug format\n    {:?}", demo2);
    print!("\n  setting level to Intermediate:");
    demo2.level = Level::Intermediate;
    print!("\n  Using Display format\n    {}", demo2);
    putline();    

    println!("\n\n  That's all Folks!\n");
}
