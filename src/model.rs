use crate::boolean::Function;
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    n: usize,
    k: usize,
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
