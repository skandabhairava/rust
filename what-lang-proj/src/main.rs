use whatlang::{detect};

fn main() {
    //let example_text = String::from("నమస్కార, నా పేరు స్కంద");
    let example_text = String::from("ನಮಸ್ಕಾರ, ನನ್ನ ಹೆಸರು ಸ್ಕಂದ");
    let info = detect(&example_text).expect("");

    println!("Language is: {}", info.lang());
    println!("Script is: {}", info.script());
    println!("INFO confidence: {}", info.confidence());
    println!("INFO reliability: {}", info.is_reliable());
}
