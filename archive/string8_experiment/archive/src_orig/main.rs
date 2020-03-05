// type Char8 = u8;
// type Char16 = u16;
// type String8 = Vec<Char8>;
// type String16 = Vec<Char16>;

struct Char<T> {
    t:T,
}

trait Collection_chars<T> {
    fn push(&mut self, ch:T);
    // fn push_str(&self, &s:String<T>);
    fn index(&mut self, indx: usize) -> &T;
}
struct String<T>{
    chars: Vec<T>,
}
impl<T> Collection_chars<T> for String<T> {
    fn push(&mut self, ch:T) {
        self.chars.push(ch)
    }
    // fn push_str(&self, &s:String<T>) {
        
    // }
    fn index(&mut self, indx: usize) -> &T {
        self.chars.get(indx)
    }
}

fn main() {

    let s8 = String8::new();
    s8.push('a' as u8);
    s8.push('b');
    println!("Hello, world!");
}
