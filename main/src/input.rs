pub mod read_input {
    use std::io;

    pub fn read_int() -> io::Result<u32> {
        let mut input = String::new();
        let stdin = io::stdin();

        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();

            if let Ok(num) = input.trim().parse::<u32>() {
                break Ok(num);
            }

            println!("Invalid input! Try again.");
        }
    }

    pub fn read_str() -> String {
        let mut input = String::new();
        let stdin = io::stdin();

        loop {
            match stdin.read_line(&mut input) {
                Ok(_) => return input.trim().to_string(),
                Err(e) => {
                    println!("Error reading input: {}. Try again.", e);
                    continue;
                }
            }
        }
    }
}
