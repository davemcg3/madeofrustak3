// used u32 instead of u8 because the decoded characters are u32 and you can't add u32 + u8
// got tripped up because we deal with negatives so i64 is the smallest that can encapsulate u32
const SHIFT: i64 = 3; // for a more complex caesar make this variable
const RADIX: u32 = 36; // 0-10a-z
const OFFSET: i64 = 10; // to offset the first 10 characters of the RADIX

fn _move_forward(letter:char) -> char {
    // for debug
    // println!("{} letter", letter);
    // println!("{} letter", letter.to_digit(RADIX).unwrap());
    // println!("{} letter + 3", letter.to_digit(RADIX).unwrap() + SHIFT);
    // println!("{} letter + 3 % 26", (letter.to_digit(RADIX).unwrap() + SHIFT - OFFSET) % 26 + OFFSET);
    // println!("{} re-encoded letter", std::char::from_digit((letter.to_digit(RADIX).unwrap() + SHIFT - OFFSET) % 26 + OFFSET, RADIX).unwrap());

    // doing some casting here because I know that I'm dealing with pretty small numbers
    std::char::from_digit(((letter.to_digit(RADIX).unwrap() as i64 + SHIFT - OFFSET) % 26 + OFFSET) as u32, RADIX).unwrap()
}

fn move_backward(letter:char) -> char {
    // for debug, there was too much debug output so I reduced it to what I needed
    // let mut debug: bool = false;
    // if letter <= 'c' { debug = true };
    // if debug {
    //     println!("{} letter", letter);
    //     println!("{} digit", letter.to_digit(RADIX).unwrap());
    //     let hello: i64 = letter.to_digit(RADIX).unwrap() as i64 - SHIFT - OFFSET;
    //     println!("{} letter - 3", hello.to_string());
    //     println!("{} + 26 (for negatives) mod 26", ((hello + 26) % 26).to_string());
    //     println!("{} letter - 3 + 26 % 26", (((letter.to_digit(RADIX).unwrap() as i64 - SHIFT - OFFSET + 26) % 26 + OFFSET) as u32).to_string());
    //     println!("{} re-encoded letter", std::char::from_digit(((letter.to_digit(RADIX).unwrap() as i64 - SHIFT - OFFSET + 26) % 26 + OFFSET) as u32, RADIX).unwrap().to_string());
    // }

    // doing some casting here because I know that I'm dealing with pretty small numbers
    std::char::from_digit(((letter.to_digit(RADIX).unwrap() as i64 - SHIFT - OFFSET + 26) % 26 + OFFSET) as u32, RADIX).unwrap()
}

pub fn _encode(plain_text: String) -> String { // used this to encode the data in SOURCE_DATA in main.rs
    let mut output: Vec<char> = vec![];
    for (_i, c) in plain_text.chars().enumerate() { // can probably get rid of the unused i on this line
        if c >= 'a' && c <= 'z' {
            output.push(_move_forward(c));
        } else {
            output.push(c);
        }
    }
    output.into_iter().collect()
}

// when I get better at this language these 2 functions are going to be aliased to reuse code
pub fn decode(encoded_text: String) -> String {
    let mut output: Vec<char> = vec![];
    for (_i, c) in encoded_text.chars().enumerate() {
        if c >= 'a' && c <= 'z' {
            output.push(move_backward(c));
        } else {
            output.push(c);
        }
    }
    output.into_iter().collect()
}