// struct_probes::main.rs

#[allow(unused_imports)]
use display::{*};
use std::fmt;

/*-- enums used for Demo1 and Demo2 --*/
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Level { Basic, Intermediate, Advanced }
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Topic { Rust, Cpp, Design, }

/*-- Struct Demo1 --*/
#[derive(Debug)]
pub struct Demo1 {
    name: String,
    level: Level,
    topic: Topic,
}
#[allow(dead_code)]
impl Demo1 {
    pub fn new() -> Self {
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

#[derive(Debug)]
pub struct Demo2 {
    pub name: String,
    pub level: Level,
    pub topic: Topic,
}
#[allow(dead_code)]
impl fmt::Display for Demo2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
          f, 
          "Demo2 {{ name: \"{}\", level: {:?}, topic: {:?} }}", 
          self.name, self.level, self.topic)
    }
}
impl Default for Demo2 {
    fn default() -> Self {
        Self {
            name: String::from(""),
            level: Level::Basic,
            topic: Topic::Rust,
        }
    }
}
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

    sub_title("Demonstrating Demo1 Struct");
    putline();
    let mut demo1 = Demo1::new();
    demo1.set_name("Demo1 probe");
    print!("\n  {:?}", demo1);
    print!("\n  Demo1 level = {:?}", demo1.get_level());
    putline();

    sub_title("Demonstrating Demo2 Struct");
    putline();
    let mut demo2 = Demo2 {
        name: String::from("Jim's Demo2"),
        ..Default::default()
    }.init();

    print!("\n  Using Display format\n    {}", demo2);
    print!("\n  setting level to Intermediate:");
    demo2.level = Level::Intermediate;
    print!("\n  Using Debug format\n    {:?}", demo2);
    putline();    

    println!("\n\n  That's all Folks!\n");
}
