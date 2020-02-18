//mod env_probe;

use std::env;

#[allow(dead_code)]
pub fn run () {

    let args :Vec<String> = env::args().collect();
    print!("\n  CLI args = {:?}\n", args);

    if args.len() > 1 {
        let arg1 = args[1].clone();
        print!("\n  arg1 {:?}\n", arg1);
    }

    let cwd = env::current_dir();
    match cwd {
        Ok(d) => print!("\n  cwd = {}\n", d.display()),
        Err(e) => print!("\n  {:?}\n", e),
    }

    let key = "PROCESSOR_IDENTIFIER";
    match env::var(key) {
        Ok(val) => print!("\n  {}: {:?}", key, val),
        Err(e) => print!("\n  couldn't interpret {}: {}", key, e),
    }
}