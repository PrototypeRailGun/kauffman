use crate::boolean::Function;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Model {
    pub n: usize,
    pub k: usize,
    pub state: Vec<bool>,
    blocks: Vec<Function>,
    structure: Vec<Vec<usize>>,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "N = {}\nK = {}\nStructure:\n{:?}\nBlocks:\n{:?}",
            self.n, self.k, self.structure, self.blocks,
        )
    }
}

impl Model {
    // Create an unconfigured model.
    pub fn new(n: usize, k: usize) -> Self {
        Model {
            n,
            k,
            state: vec![],
            blocks: vec![],
            structure: vec![],
        }
    }

    // Set functions of logical blocks.
    pub fn set_functions(&mut self, func: Vec<Function>) {
        self.blocks = func;
    }

    // Set links between blocks.
    pub fn set_structure(&mut self, structure: Vec<Vec<usize>>) {
        self.structure = structure;
    }

    // Iterate the model.
    pub fn update(&mut self) {
        self.state = (0..self.n)
            .map(|i| {
                self.blocks[i].eval(self.structure[i].iter().map(|&s| self.state[s]).collect())
            })
            .collect();
    }
}

// Create a random model with parameters n and k;
pub fn random_model(n: usize, k: usize) -> Model {
    let mut model = Model::new(n, k);
    let mut rng = rand::thread_rng();

    let structure: Vec<Vec<usize>> = (0..n)
        .map(|i| {
            let mut src: Vec<usize> = Vec::new();
            while src.len() < k {
                let r = rng.gen_range(0..n);
                if r != i {
                    src.push(r);
                }
            }
            src
        })
        .collect();

    model.set_structure(structure);
    model
}

// Read the structure of the model from the keyboard
pub fn read_model(n: usize, k: usize) -> Model {
    let mut model = Model::new(n, k);
    let mut buf = String::new();
    let mut structure: Vec<Vec<usize>> = Vec::with_capacity(k);

    for i in 0..n {
        let mut source: Vec<usize> = Vec::with_capacity(k);

        'outer: loop {
            source.clear();
            buf.clear();

            println!("Enter the source indexes for block {}:", i);
            std::io::stdin().read_line(&mut buf).unwrap();

            for x in buf.split_ascii_whitespace() {
                match x.parse::<usize>() {
                    Ok(num) => source.push(num),
                    Err(_) => {
                        println!("Invalid character. Try again.");
                        continue 'outer;
                    }
                }
            }

            if source.len() == k {
                break;
            } else {
                println!(
                    "Incorrect number of indexes entered: {} instead {}",
                    source.len(),
                    k,
                );
            }
        }

        structure.push(source);
    }

    model.set_structure(structure);
    model
}
