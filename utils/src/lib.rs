use std::{io::{self, Write}, error::Error};


pub fn input(question: &str) -> Result<String, Box<dyn Error>> {

    let mut guess = String::new();

    print!("{} ", question);

    io::stdout().flush()?;
    io::stdin().read_line(&mut guess)?;
    
    Ok(guess.trim().to_string())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
