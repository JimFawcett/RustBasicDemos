// RustBasicDemos::iterator_probes


fn main() {

    let v = vec![1, -1, 2, -2, 3, -3];
    print!("\n  ");
    let it = v.iter();
    for val in it {
        print!("{} ", val);
        }
    }

    println!("\n  Hello, world!");
}
