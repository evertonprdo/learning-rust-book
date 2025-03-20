use std::io;

pub fn request_valid_u8() -> u8 {
    loop {
        match stdin_str().parse::<u8>() {
            Ok(val) => {
                return val;
            }
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
    }
}

pub fn stdin_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    return s.trim().to_string();
}
