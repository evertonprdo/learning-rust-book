use std::{io, ops::Range};

// The first goal is to compute the bounds of a signed bit-{x}
// currently it can compute up to 32 bits
// The formula supports more bits, but to print a larger bound, another approach will be needed

fn main() {
    println!("Choose a bit length by index:");
    let mut opts: [u8; 3] = [0; 3];

    for i in 0..3 {
        let bit_length = 8 * 2_u8.pow(i);
        opts[i as usize] = bit_length;

        println!("{i}. {}-bit", bit_length);
    }

    let bit_length = stdin_number(0..5);

    println!(
        "Calculating range of signed {}-bit integer",
        opts[bit_length as usize]
    );

    let (from, to) = calculate_signed_range(&(opts[bit_length as usize] as u32));
    println!("It has a minimum value of {from} and a maximum value of {to}");
}

fn stdin_number(range: Range<u8>) -> u8 {
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse::<u8>() {
            Ok(num) => {
                if range.contains(&num) {
                    return num;
                }
            }

            Err(_) => {}
        };

        println!(
            "Please enter a valid number between {} and {}",
            range.start, range.end
        );
    }
}

fn calculate_signed_range(bit_length: &u32) -> (i64, i64) {
    let exponent: u32 = *bit_length - 1;

    let from = -2_i64.pow(exponent);
    let to = 2_i64.pow(exponent) - 1;

    return (from, to);
}
