use std::collections::HashMap;
use std::io::ErrorKind;
use std::{fs, io, env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2{
        println!("No file argument given for config!");
        process::exit(1);
    }
    let file_name = args.get(1).unwrap();

    let config = match read_config(&file_name){
        Ok(hash) => {
            println!("Config has been read!");
            hash
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!("File {:?} Not found!", file_name),
                ErrorKind::InvalidData => println!("Cannot parse file {:?}!", file_name),
                unknown => println!("Error: {:?}", unknown),
            }
            process::exit(1);
        },
    };

    println!("{:?}", config);
}

fn read_config(file_name: &str) -> Result<HashMap<String, String>, io::Error>{
    let string = fs::read_to_string(&file_name)?;
    let lines:Vec<&str> = string.lines().collect();

    let mut dict = HashMap::new();

    for elem in lines{
        let line:Vec<&str> = elem.split('=').collect();
        if line.len() != 2{
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        dict.insert(line.get(0).unwrap_or(&"").to_string(), line.get(1).unwrap_or(&"").to_string());
    }

    return Ok(dict);
}