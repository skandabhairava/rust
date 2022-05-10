use std::fmt;

///////////////////////////////////////////////////////////
struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
///////////////////////////////////////////////////////////

struct Age(u32);
struct ID(u32);

fn main() {
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);
    let (_age, _id) = (Age(4), ID(5));
}
