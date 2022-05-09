use traits::{Draw, Screen, Button};

///////////////////////////////////////////////////////////////////////
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
    impl Draw for SelectBox {
        fn draw(&self) {
            //draw select box
        }
    }
///////////////////////////////////////////////////////////////////////

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(
                SelectBox {
                    width: 100,
                    height: 200,
                    options: vec![
                        String::from("Yes!"),
                        String::from("No!"),
                        String::from("Maybe!"),
                    ],
                }
            ),
            Box::new(
                Button {
                    width: 100,
                    height: 100,
                    label: String::from("Ok!")
                }
            ),
        ],
    };
}
