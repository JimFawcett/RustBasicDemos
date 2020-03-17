/// ////////////////////////////////////////////////////////
/// logger::main.rs - log strings to file, console        //
///                                                       //
/// Jim Fawcett, https://JimFawcett.github,io, 3/16/2020  //
/// ////////////////////////////////////////////////////////

extern crate chrono;
use chrono::{DateTime, Local};

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
    ///--- fl:None, console:true
    pub fn new() -> Self {
        Self { fl:None, console:true, }
    }
    ///--- set file, console
    pub fn init(&self, f:File, con:bool) -> Self {
        Self { fl:Some(f), console:con, }
    }
    ///--- set console true | false
    pub fn console(&mut self, con:bool) {
        self.console = con
    }
    ///--- set | reset file
    pub fn file(&mut self, f:File) {
        self.fl = Some(f);
    }
    fn opt(&mut self, f:Option<File>) {
        self.fl = f;
    }
    ///--- attempt to open logger file with filename s
    pub fn open(&mut self, s:&str) {
        use std::fs::OpenOptions;
        self.fl = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(s).ok();
    }
    ///--- write s with time stamp
    pub fn ts_write(&mut self, s:&str) {
        let now: DateTime<Local> = Local::now();
        print!("\n  {}", now);
        Logger::write(self, s);
    }
    ///--- write s
    pub fn write(&mut self, s:&str) {
        if let Some(ref mut fl) = self.fl {
            let _n = fl.write(s.as_bytes());
        }
        if self.console {
            print!("{}", s);
        }
    }
    ///--- close file
    pub fn close(&mut self) {
        self.fl = None;
    }
}
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum OpenMode { Truncate, Append }
#[allow(dead_code)]
///--- attempt to open file with name str
pub fn open_file(s:&str, mode:OpenMode) -> Option<File> {
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
    let f:Option<File> = open_file(file_name, OpenMode::Append);

    let mut log = Logger::new();
    log.console(true);
    log.opt(f);
    log.ts_write("\n  starting log");
    log.write("\n  first entry");
    log.write(", second entry");
    log.close();
    log.console(false);
    /*-- won't write, won't panic --*/
    log.write("\n  after close");
    /*-- reopens logger with named file --*/
    log.open("log.txt");
    log.console(true);
    log.ts_write("\n  reopening log");
    log.write("\n  after reopen");

    println!("\n\n  That's all Folks!\n");
}
