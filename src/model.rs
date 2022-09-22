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

    let mut matrix: Vec<Vec<u16>> = vec![vec![0; n]; n];

    for i in 0..n {
        let mut col_sums: Vec<u16> = vec![0; n];
        for j in 0..n {
            col_sums[j] = (0..n).map(|r| matrix[r][j]).sum::<u16>();
        }

        let mut tg_col_sum = col_sums.iter().map(|&x| x).min().unwrap();
        let mut filled = 0;

        while filled < k {
            let mut val: Vec<usize> = (0..n)
                .filter(|&idx| idx != i && col_sums[idx] == tg_col_sum)
                .collect();

            while val.len() > 0 && filled < k {
                let idx = rng.gen_range(0..val.len());
                matrix[i][val[idx]] = 1;
                val.remove(idx);
                filled += 1;
            }

            tg_col_sum += 1;
        }
    }

    for row in matrix.iter() {
        println!("{:?}", row);
    }

    let structure: Vec<Vec<usize>> = matrix
        .into_iter()
        .map(|row| (0..n).filter(|&j| row[j] == 1).collect())
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
