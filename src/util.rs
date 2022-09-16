use crate::model::Model;
use std::fs::File;
use std::io::{Read, Write};

pub fn input_helper(message: &str, req: Vec<&str>) -> String {
    let mut buf = String::new();
    loop {
        println!("{}", message);
        std::io::stdin().read_line(&mut buf).unwrap();
        buf = buf.trim().to_ascii_uppercase();
        if req.contains(&buf.as_str()) {
            return buf;
        } else {
            println!("Incorrect input! Try again.");
        }
        buf.clear();
    }
}

pub fn get_usize(message: &str) -> usize {
    let mut buf = String::new();
    loop {
        println!("{}", message);
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Enter a number!"),
        }
        buf.clear();
    }
}

pub fn serialize_model(model: &Model) -> std::io::Result<()> {
    let serialized = serde_json::to_string(model).unwrap();

    let mut path = String::new();
    println!("Specify the path to the file to store the model:");
    std::io::stdin().read_line(&mut path)?;

    let mut file = File::create(path.trim())?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn deserialize_model() -> std::io::Result<Model> {
    let mut path = String::new();
    println!("Specify the path to the file with the serialized model:");
    std::io::stdin().read_line(&mut path)?;

    let mut file = File::open(path.trim())?;
    let mut serialized = String::new();
    file.read_to_string(&mut serialized)?;

    Ok(serde_json::from_str(&serialized).unwrap())
}

// Read the initial state of the model
pub fn read_init_state(n: usize) -> Vec<bool> {
    let mut buf = String::new();
    let mut values: Vec<bool> = Vec::with_capacity(n);

    'outer: loop {
        values.clear();
        buf.clear();

        println!(
            "Enter the initial values at the outputs of the model blocks, separated by a space:"
        );
        std::io::stdin().read_line(&mut buf).unwrap();

        for x in buf.split_ascii_whitespace() {
            match x.parse::<u8>() {
                Ok(num) => values.push(num != 0),
                Err(_) => {
                    println!("Invalid character. Try again.");
                    continue 'outer;
                }
            }
        }

        if values.len() == n {
            return values;
        } else {
            println!(
                "Incorrect number of values entered: {} instead {}",
                values.len(),
                n,
            );
        }
    }
}
