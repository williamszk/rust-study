// Build an Affine Cipher

use std::collections::HashMap;

fn main() {
    generate_dictionaries_for_encoding_and_decoding();

    let alpha = 9;
    let beta = 2;
    let x = 0;

    // plaintext: affine
    // ciphertext: CVVWPM
    let plaintext = String::from("affine");

    let ciphertext = encrypt_affine(plaintext, alpha, beta);
}

// create a closure
// I want to have the definition of the dict_dcript hashmap for some of the other
// functions to use, but I don't want to run it every time that I need to call it
// in this case I can build a closure
// the closure is a function that will run once in the program
// and the closure will output another function which is the function with
// the defined values were already calculated
// the idea above didn't go forward, even if it was a good idea

/// This function receives a String and output another String.
/// It will encrypt the plaintext into ciphertext using
/// the affine cipher.
fn encrypt_affine(plaintext: String, alpha: i32, beta: i32) -> String {
    println!("len {}", plaintext.len());
    let mut i: i32 = 0;
    let mut letter: char;
    let mut x: i32;
    loop {
        letter = plaintext.chars().nth(i as usize).unwrap();

        x = encode_char_to_num(letter);

        let y = affine_cipher(x, alpha, beta);



        println!("{}", x);
        i += 1;

        if i == plaintext.len() as i32 {
            break;
        }
    }

    let ciphertext: String = String::from("");

    ciphertext
}

/// This function receives a String and output another String.
/// It will decrypt the ciphertext into plaintext using
/// the affine cipher.
fn decrypt_affine(ciphertext: String, alpha: i32, beta: i32) -> String {
    let plaintext: String = String::from("");


    plaintext
}

fn decode_num_to_char(y: i32) -> char {

    let dict_decode: HashMap<i32, char> = HashMap::from([
        (4, 'E'),
        (1, 'B'),
        (17, 'R'),
        (19, 'T'),
        (10, 'K'),
        (3, 'D'),
        (16, 'Q'),
        (2, 'C'),
        (0, 'A'),
        (11, 'L'),
        (7, 'H'),
        (8, 'I'),
        (5, 'F'),
        (14, 'O'),
        (15, 'P'),
        (18, 'S'),
        (12, 'M'),
        (20, 'U'),
        (23, 'X'),
        (24, 'Y'),
        (25, 'Z'),
        (9, 'J'),
        (22, 'W'),
        (21, 'V'),
        (13, 'N'),
        (6, 'G'),
    ]);

    let letter = dict_decode.get(&y);

    letter
}

fn encode_char_to_num(letter: char) -> Option<&i32> {
    let dict_encode: HashMap<char, i32> = HashMap::from([
        ('f', 5),
        ('c', 2),
        ('v', 21),
        ('d', 3),
        ('b', 1),
        ('m', 12),
        ('o', 14),
        ('l', 11),
        ('g', 6),
        ('a', 0),
        ('q', 16),
        ('s', 18),
        ('t', 19),
        ('i', 8),
        ('k', 10),
        ('n', 13),
        ('u', 20),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('e', 4),
        ('h', 7),
        ('r', 17),
        ('j', 9),
        ('p', 15),
        ('z', 25),
    ]);

    let x = dict_encode.get(&letter);

    x
}


/// This function is more for reference purpose.
/// We built the HashMaps as literals in another function.
fn generate_dictionaries_for_encoding_and_decoding() {
    // how to build a vector of alphabet letters in rust?
    // https://stackoverflow.com/a/45344045/8782077

    let alphabet = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    println!("The alphabet lower case: {:?}", alphabet);

    let mut dict_encrypt: HashMap<char, i32> = HashMap::new();
    let mut i = 0;
    for letter in alphabet {
        dict_encrypt.insert(letter, i);
        i += 1;
    }
    println!("dict_encrypt: {:#?}", dict_encrypt);

    let alphabet_upper = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    println!("The alphabet upper case: {:?}", alphabet_upper);

    let mut dict_decrypt: HashMap<i32, char> = HashMap::new();
    let mut i = 0;
    for letter in alphabet_upper {
        dict_decrypt.insert(i, letter);
        i += 1;
    }

    println!("dict_decrypt: {:#?}", dict_decrypt);
}

fn affine_cipher(x: i32, alpha: i32, beta: i32) -> i32 {
    let y = alpha * x + beta;
    y
}
