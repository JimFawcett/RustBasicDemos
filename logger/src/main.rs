// logger::main.rs

#[allow(unused_imports)]
use display::{*};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Logger {
    fl:Option<File>,
    console:bool,
}
#[allow(dead_code)]
impl Logger {
    pub fn new() -> Self {
        Self { fl:None, console:false, }
    }
    pub fn init(&self, f:File, con:bool) -> Self {
        Self { fl:Some(f), console:con, }
    }
    pub fn console(&mut self, con:bool) {
        self.console = con
    }
    pub fn file(&mut self, f:File) {
        self.fl = Some(f);
    }
    pub fn opt(&mut self, f:Option<File>) {
        self.fl = f;
    }
    pub fn open(&mut self, s:&str) {
        use std::fs::OpenOptions;
        self.fl = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(s).ok();
    }
    pub fn write(&mut self, s:&str) {
        if let Some(ref mut fl) = self.fl {
            let _n = fl.write(s.as_bytes());
        }
        if self.console {
            print!("{}", s);
        }
    }
    pub fn close(&mut self) {
        self.fl = None;
    }
}
#[derive(PartialEq)]
#[allow(dead_code)]
enum OpenMode { Truncate, Append }
#[allow(dead_code)]
fn open_file(s:&str, mode:OpenMode) -> Option<File> {
    let fl:Option<File>;
    use std::fs::OpenOptions;
    if mode == OpenMode::Truncate {
        fl = OpenOptions::new()
             .write(true)
             .truncate(true)
             .open(s).ok();
    }
    else {
        fl = OpenOptions::new()
             .write(true)
             .create(true)
             .append(true)
             .open(s).ok();
    }
    fl
}

fn main() {

    main_title("Demonstrating Logger");
    putline();
    sub_title("written to \"Log.txt\"");

    let file_name = "log.txt";
    let f = open_file(file_name, OpenMode::Append);

    let mut log = Logger::new();
    log.console(true);
    log.opt(f);
    log.write("\n  first entry");
    log.write(", second entry");
    log.close();
    log.console(false);
    /*-- won't write, won't panic --*/
    log.write("\n  after close");
    /*-- reopens logger with named file --*/
    log.open("log.txt");
    log.console(true);
    log.write("\n  after reopen");

    println!("\n\n  That's all Folks!\n");
}
