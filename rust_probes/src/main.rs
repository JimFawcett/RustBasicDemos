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
mod probe_traits;
mod probe_unsafe;
mod ownership;
mod types;

fn main() {

    //formatstructure::run();
    //dblformats::run();
    //debugformats::run();
    //demodrop::run();
    //env_probe::run();
    //probe_struct::run();
    probe_traits::run();
    //probe_unsafe::run();
    //ownership::run();
    //types::run();
    print!("\n\n");
}
