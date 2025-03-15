use std::{io, ops::Range};

// The first goal is to compute the bounds of a signed or unsigned bit-{x}
// currently it can compute up to 32 bits
// The formula supports more bits, but to print a larger bound, another approach will be needed

fn main() {
    println!("Choose a bit length by index:");
    let opts = [8, 16, 32];
    let mut i = 0;

    for opt in opts {
        println!("{i}. {}-bit", opt);
        i += 1;
    }

    let bit_length = stdin_number(0..opts.len());

    println!("Calculate for:\n0. unsigned\n1. signed");
    let is_signed_bit = stdin_number(0..2) != 0;

    println!(
        "Calculating range of {} {}-bit integer...",
        if is_signed_bit { "signed" } else { "unsigned" },
        opts[bit_length]
    );

    let (from, to) = if is_signed_bit {
        calculate_signed_range(&opts[bit_length])
    } else {
        calculate_unsigned_range(&opts[bit_length])
    };

    println!("It has a minimum value of {from} and a maximum value of {to}");
}

fn stdin_number(range: Range<usize>) -> usize {
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse::<usize>() {
            Ok(num) => {
                if range.contains(&num) {
                    return num;
                }
            }

            Err(_) => {}
        };

        println!(
            "Please enter a valid number between {} and {}",
            range.start,
            range.end - 1
        );
    }
}

fn calculate_unsigned_range(bit_length: &u32) -> (i64, i64) {
    return (0, 2_i64.pow(*bit_length) - 1);
}

fn calculate_signed_range(bit_length: &u32) -> (i64, i64) {
    let exponent: u32 = *bit_length - 1;

    let from = -2_i64.pow(exponent);
    let to = 2_i64.pow(exponent) - 1;

    return (from, to);
}
