use std::ops::Deref;

////////////////////////////////////////////////////////////
struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
////////////////////////////////////////////////////////////

fn main() {
    //dereferencing

    /* let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, y); */

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str (dereferencing)
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}