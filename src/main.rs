mod boolean;
mod util;

use util::{get_usize, input_helper};

fn main() {
    let load_state = input_helper("Use an existing model? (Y/n): ", vec!["Y", "N"]);

    if load_state == "Y" {
        println!("Deserialization...");
    } else {
        let n = get_usize("Number of logical blocks: N = ");
        let k = get_usize("Number of block inputs: K = ");

        let conf_mode = input_helper(
            "Select the configuration mode of the model structure (Manual or Automatic). Input M/A: ",
            vec!["M", "A"],
        );

        // Инициализация ненастроенного автомата с параметрами N и K

        if conf_mode == "A" {
            println!("Случайная генерация автомата...");
        } else {
            println!("Ручной ввод связей...");
        }

        let func_conf_mode = input_helper(
            "Select the input mode for logical block functions (Manual or Automatic). Input M/A: ",
            vec!["M", "A"],
        );

        if func_conf_mode == "A" {
            println!("Автоматическое создание функций...");
        } else {
            println!("Ручной ввод функций...")
        }
    }

    // Вывод матрицы автомата и используемых логических функций

    let ser_state = input_helper("Serialize the model? (Y/n): ", vec!["Y", "N"]);
    if ser_state == "Y" {
        println!("Serialization...");
    }

    let runs: usize = get_usize("Number of different initial states: ");

    let inst_inp_mode = input_helper(
        "Select the initial state input mode (Manual or Automatic). Input M/A: ",
        vec!["M", "A"],
    );

    if inst_inp_mode == "M" {
        println!("Ручной ввод начальных состояний");
    } else {
        println!("Автоматический ввод начальных состояний");
    }

    println!("Результат...");
}
