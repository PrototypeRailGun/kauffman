mod boolean;
mod model;
mod util;

use boolean::{random_function, read_function, Function};
use model::{random_model, read_model, Model};
use std::collections::HashSet;
use util::{deserialize_model, get_usize, input_helper, serialize_model};

fn main() {
    let load_state = input_helper("Use an existing model? (Y/n): ", vec!["Y", "N"]);
    let mut model: Model;

    if load_state == "Y" {
        match deserialize_model() {
            Ok(m) => model = m,
            Err(msg) => {
                println!("Failed to load the model: {}", msg);
                return;
            }
        }
    } else {
        let n = get_usize("Number of logical blocks: N = ");
        let k = get_usize("Number of block inputs: K = ");

        let conf_mode = input_helper(
            "Select the configuration mode of the model structure (Manual or Automatic). Input M/A: ",
            vec!["M", "A"],
        );

        if conf_mode == "A" {
            model = random_model(n, k);
        } else {
            model = read_model(n, k);
        }

        let func_conf_mode = input_helper(
            "Select the input mode for logical block functions (Manual or Automatic). Input M/A: ",
            vec!["M", "A"],
        );

        let mut functions: Vec<Function> = Vec::with_capacity(n);
        if func_conf_mode == "A" {
            for _ in 0..n {
                functions.push(random_function(k));
            }
        } else {
            for _ in 0..n {
                functions.push(read_function(k));
            }
        }
        model.set_functions(functions);
    }

    println!("Model:");
    println!("{}", model);

    let ser_state = input_helper("Serialize the model? (Y/n): ", vec!["Y", "N"]);
    if ser_state == "Y" {
        match serialize_model(&model) {
            Ok(_) => println!("Serialized successfully!"),
            Err(msg) => println!("Error: {}", msg),
        }
    }

    let mut attractors: Vec<HashSet<Vec<bool>>> = Vec::new();

    for mut x in 0..2usize.pow(model.n as u32) {
        let mut init_state: Vec<bool> = Vec::with_capacity(model.n);
        while x > 0 {
            init_state.push(x % 2 != 0);
            x /= 2;
        }
        for _ in 0..(model.n - init_state.len()) {
            init_state.push(false);
        }
        model.state = init_state;

        let mut states: Vec<Vec<bool>> = Vec::new();
        loop {
            match (0..states.len())
                .filter(|&i| states[i] == model.state)
                .next()
            {
                None => {
                    states.push(model.state.clone());
                    model.update();
                }
                Some(idx) => {
                    let attr: HashSet<Vec<bool>> = states.into_iter().skip(idx).collect();
                    if !attractors.contains(&attr) {
                        attractors.push(attr);
                    }
                    break;
                }
            }
        }
    }

    println!("\nNumber of attractors M = {}", attractors.len());
    println!(
        "Average length of the attractor L = {}",
        (attractors.iter().map(|attr| attr.len()).sum::<usize>() as f64)
            / (attractors.len() as f64)
    );
}
