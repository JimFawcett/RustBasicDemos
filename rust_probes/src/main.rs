/////////////////////////////////////////////////////////////
// main.rs - executes named module                         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 24 Feb 2020  //
/////////////////////////////////////////////////////////////

mod formatstructure;
mod dblformats;
mod debugformats;
mod demodrop;
mod env_probe;
mod probe_struct;
mod ownership;
mod types;

fn main() {

    //formatstructure::run();
    //dblformats::run();
    //debugformats::run();
    //demodrop::run();
    //env_probe::run();
    //probe_struct::run();
    ownership::run();
    //types::run();
    print!("\n\n");
}
