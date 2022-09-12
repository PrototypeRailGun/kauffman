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
