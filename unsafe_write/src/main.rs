static HELLO_WORLD: &str = "Hello, world";

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe fn dangerous() {}

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
    }

    println!("{}", HELLO_WORLD);
}