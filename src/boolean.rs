use rand;
use serde::{Deserialize, Serialize};

// A boolean function of k variables with one output.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Function {
    k: usize,
    values: Vec<bool>,
}

impl Function {
    pub fn new(k: usize, values: Vec<bool>) -> Self {
        Function { k, values }
    }

    // Find out the value on a given arguments
    pub fn eval(&self, variables: Vec<bool>) -> bool {
        let mut row = 0usize;
        for (i, v) in variables.into_iter().rev().enumerate() {
            if v {
                row += 2usize.pow(i as u32);
            }
        }
        self.values[row]
    }
}

// Randomly generates a column of boolean function values from k variables.
pub fn random_function(k: usize) -> Function {
    Function::new(
        k,
        (0..2usize.pow(k as u32)).map(|_| rand::random()).collect(),
    )
}

// Read a function from k variables from the keyboard.
// A string of 2^k numbers 0 or 1 separated by a space is entered.
pub fn read_function(k: usize) -> Function {
    let target = 2usize.pow(k as u32);
    let mut buf = String::new();
    let mut values: Vec<bool> = Vec::with_capacity(target);

    'outer: loop {
        values.clear();
        buf.clear();

        println!(
            "Enter a column of boolean function values (a string of { } numbers 0 or 1 separated by a space):",
            target,
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

        if values.len() == target {
            return Function::new(k, values);
        } else {
            println!(
                "Incorrect number of values entered: {} instead {}",
                values.len(),
                target,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_function() {
        let f = random_function(3);
        println!("{:?}", f);
        println!("{:?}", f.eval(vec![false, false, false]));
        println!("{:?}", f.eval(vec![false, false, true]));
        println!("{:?}", f.eval(vec![false, true, false]));
        println!("{:?}", f.eval(vec![false, true, true]));
        println!("{:?}", f.eval(vec![true, false, false]));
        println!("{:?}", f.eval(vec![true, false, true]));
        println!("{:?}", f.eval(vec![true, true, false]));
        println!("{:?}", f.eval(vec![true, true, true]));
    }

    #[test]
    fn test_read_function() {
        let f = read_function(2);
        println!("{:?}", f);
    }
}
