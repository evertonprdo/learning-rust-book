/*  https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
    so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
    Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    let message = String::from("Hello world into Pig Latin");
    let mut words: Vec<String> = message.split_whitespace().map(|w| w.to_string()).collect();
    let mut iter = words.iter_mut();

    while let Some(word) = iter.next() {
        *word = word_to_pig_latin(&word);
    }

    println!("{}\n{}", message, words.join(" "))
}

fn word_to_pig_latin(s: &str) -> String {
    let mut ascii_iter = s.bytes();
    let fst_char = if let Some(c) = ascii_iter.next() {
        c.to_ascii_lowercase()
    } else {
        return String::new();
    };

    let is_vowel = ASCII_VOWELS.contains(&fst_char);

    let mut i: usize = 0;
    let mut ascii_bytes: Vec<u8> = vec![0; if is_vowel { s.len() + 4 } else { s.len() + 3 }];

    if is_vowel {
        ascii_bytes[i] = fst_char;
        i += 1;
    }

    for c in ascii_iter {
        ascii_bytes[i] = c;
        i += 1;
    }

    ascii_bytes[i] = 45; //      -
    ascii_bytes[i + 1] = if is_vowel { 104 } else { fst_char };
    ascii_bytes[i + 2] = 97; //  a
    ascii_bytes[i + 3] = 121; // y

    return String::from_utf8(ascii_bytes).unwrap();
}

// [a, b, c, d, e]
const ASCII_VOWELS: [u8; 5] = [97, 101, 105, 111, 117];
