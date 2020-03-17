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
    let file_name = "log.txt";

    if file_exists(file_name) {
        remove_file(file_name);
        print!("\n  deleted previous log file");
    }
    else {
        print!("\n  failed to delete previous log file");
    }

    sub_title("writing to \"Log.txt\"");

    let f:Option<File> = open_file(file_name, OpenMode::Append);

    let mut log = Logger::new();
    log.console(true);
    log.opt(f);
    log.ts_write("\n  starting log");
    log.write("\n  first entry").write(", second entry");
    log.write(", third entry");
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
