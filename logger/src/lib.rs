/// ////////////////////////////////////////////////////////
/// logger::lib.rs - log strings to file, console         //
///                                                       //
/// Jim Fawcett, https://JimFawcett.github,io, 3/16/2020  //
/// ////////////////////////////////////////////////////////

extern crate chrono;
#[allow(unused_imports)]
use chrono::{DateTime, Local};

#[allow(unused_imports)]
use display::{*};
use std::fs::File;
use std::path::{*};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Logger {
    fl:Option<File>,
    console:bool,
}
#[allow(dead_code)]
impl Logger {
    /// ```
    /// let mut logr = Logger::new();
    ///
    /// sets fl:None, console:true;
    /// ```
    pub fn new() -> Self {
        Self { fl:None, console:true, }
    }
    /// ```
    /// let mut logr = Logger::init(file, true);
    ///
    /// sets fl:Some(file), console:true;
    /// ```
    pub fn init(&self, f:File, con:bool) -> Self {
        Self { fl:Some(f), console:con, }
    }
    /// ```
    /// logr.console(pred);
    ///
    /// sets console:pred;
    /// ```
    pub fn console(&mut self, con:bool) {
        self.console = con
    }
    /// ```
    /// logr.file(file);
    ///
    /// sets | resets fl:Some(file);
    /// ```
    pub fn file(&mut self, f:File) {
        self.fl = Some(f);
    }
    /// ```
    /// logr.opt(opt);
    ///
    /// sets | resets fl:opt;
    /// ```
    pub fn opt(&mut self, f:Option<File>) {
        self.fl = f;
    }
    /// ```
    /// logr.open(file_name);
    ///
    /// attempts to set fl:Some(file)
    /// ```
    pub fn open(&mut self, s:&str) {
        use std::fs::OpenOptions;
        self.fl = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(s).ok();
    }
    /// ```
    /// logr.ts_write(some_str);
    ///
    /// writes local time stamp and some_str to log, can be chained
    /// ```
    pub fn ts_write(&mut self, s:&str) -> &mut Self {
        
        let now: DateTime<Local> = Local::now();
        /* format DateTime string */
        let mut now_str = format!("\n  {}", now.to_rfc2822());
        /* remove trailing -0400 */
        now_str.truncate(now_str.len() - 6);

        Logger::write(self, &now_str);
        Logger::write(self, s);
        self
    }
    /// ```
    /// logr.write(some_str);
    ///
    /// writes some_str to log, can be chained
    /// ```
    pub fn write(&mut self, s:&str) -> &mut Self {
        if let Some(ref mut fl) = self.fl {
            let _n = fl.write(s.as_bytes());
        }
        if self.console {
            print!("{}", s);
        }
        self
    }
    /// ```
    /// logr.close();
    ///
    /// sets fl:None
    /// ```
    pub fn close(&mut self) {
        self.fl = None;
    }
}
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum OpenMode { Truncate, Append }
#[allow(dead_code)]

    /// ```
    /// let f:Option<File> = open_file(some_string, Append);
    ///
    /// attempts to open file with specified OpenMode: Truncate | Append
    /// ```
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
    /// ```
    /// if remove_file(file_name) { ... }
    /// ```
pub fn remove_file(s:&str) -> bool {
    let rslt = std::fs::remove_file(s);
    rslt.is_ok()
}
    /// ```
    /// if file_exists(file_name) { ... }
    /// ```
    pub fn file_exists(s:&str) -> bool {
    let path = Path::new(s);
    return path.exists();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_file() {
        let stest = "test_remove";
        open_file(stest, OpenMode::Truncate);
        remove_file(stest);
        assert_eq!(file_exists(stest),false);
    }
    #[test]
    fn test_file_exists() {
        assert_eq!(file_exists("foobar.fee"),false);
    }
    #[test]
    fn test_new() {
        let stest = "test_new";
        let mut l = Logger::new();
        l.open(stest);
        assert_eq!(file_exists(stest), true);
        remove_file(stest);
        assert_eq!(file_exists(stest), false);
    }
}
