/// -------------------------------------------------------
/// RustBasicDemos::Logger::examples::test1.rs
/// Jim Fawcett, https://JimFawcett.github.io, 17 Mar 2020
/// -------------------------------------------------------

extern crate logger;

use logger::{*};
use display::{*};
use std::fs::File;

fn main() {

    main_title("Demonstrating Logger");
    putline();
    let log_file_name = "log.txt";

    let err_file_name = "err.txt";
    let mut err_log = Logger::new();
    err_log.open(&err_file_name);
    err_log.ts_write("\n  start log");

    if file_exists(log_file_name) {
        if remove_file(log_file_name) {
            print!("\n  deleted previous log file");
        }
        else {
            err_log.write("\n  can't delete previous log file");
        }
    }
    else {
        print!("\n  log file does not exist");
    }

    sub_title("writing to \"Log.txt\"");

    let f:Option<File> = open_file(log_file_name, OpenMode::Append);
    if let None = f {
        err_log.write("\n  Open_file failed");
    } 
    let mut log = Logger::new();
    log.console(true);
    log.opt(f);
    log.ts_write("\n  starting log");
    log.write("\n  first entry").write(", second entry");
    log.write(", third entry");
    log.close();
    putline();
    
    let s = format!("contents of {}", log_file_name);
    sub_title(&s);
    file_contents(log_file_name);
    putline();

    sub_title("closing then reopening log");
    log.console(false);
    /*-- won't write, won't panic --*/
    log.write("\n  after close");
    /*-- reopens logger with named file --*/
    log.open_append("log.txt");
    log.console(true);
    log.ts_write("\n  reopening log");
    log.write("\n  after reopen");
    log.close();
    
    let test_string = "first entry";
    let b = file_contains(log_file_name, test_string);
    if b {
        print!("\n  found {:?} in {:?}", test_string, log_file_name);
    }
    else {
        let s = format!("\n  did not find {:?} in {:?}", test_string, log_file_name);
        err_log.write(&s);
    }
    println!("\n\n  That's all Folks!\n");
}
