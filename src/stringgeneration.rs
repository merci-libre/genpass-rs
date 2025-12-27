/*
* [stringgeneration.rs]
*
* This is the password engine used in genpass-rs. It uses the CSPRNG of the rand
* crate to generate randomized strings. It supports all UTF-8 character generation
* up to 2^8 (0-255).
*
* There are seperate functions for separate needs, for example-- the first function below: `fn generator()`
* specifically creates passwords with the highest amount of entropy that this program can allow.
*
* UTF-8 umlauts and other accent marks are appended onto vowels, but also can just directly be
* referrenced directly. This program's password generation technique takes advantage of this when
* generating random UTF-8 in the values above 127 (standard ascii range).
*
* */

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{io, thread, time, u8};
/* As of v.1.0.5, you don't need to uncomment any code for debugging purposes.*/
fn testing(string: String, char_value: u8, char_count: i16, target_bytesize: i16, truecount: u16) {
    let mut t = term::stdout().unwrap();
    writeln!(t,"\nCurrently Generated String:\n{string}\ncurrent_bytesize={char_count}\ntarget_bytesize={target_bytesize}\ncharacter value: {char_value}\ntruecount={truecount}").unwrap();
    thread::sleep(time::Duration::from_millis(200));

    // to make terminal output look nice:
    for _i in 0..7 {
        t.carriage_return().unwrap();
        t.delete_line().unwrap();
        t.reset().unwrap();
        t.cursor_up().unwrap();
    }
    io::Write::flush(&mut io::stdout()).expect("failed to flush output");
}

pub fn generator(
    /* generates a CSPRNG safe string.
     * may be rewritten as a method.
     * */
    length: u8, // length of the password, cannot exceed 255 and must be an 8-bit unsigned integer
    char_min: u8, // minimum value that the generator can generate. Must be an 8-bit unsigned
    // integer.
    char_max: u8, // maximum value that the generator can generate. Must be an 8-bit unsigned
    // integer.
    mut string: String, // inputted string, must be mutable and of type "String".
    debug: bool, // Show debugging information, and real-time generation. Takes a boolean value.
) -> String {
    let mut bytesize: i16 = 0;
    let mut target_bytesize: i16 = length.into();
    let max_size: i16 = target_bytesize * 2;
    let mut truecount: u16 = 0;

    // generate character list to pull from.
    let mut charlist: Vec<char> = Vec::new();

    for i in char_min..127 {
        charlist.push(i as char);
    }
    // generates extasc if set as a parameter, and with a 'pwetty pwease'
    if char_max == 255 {
        for i in 161..u8::MAX {
            charlist.push(i as char);
        }
    }

    let mut random = StdRng::from_os_rng();

    while bytesize != target_bytesize {
        // continuously generates random characters until the target_bytesize is reached.
        let x: usize = random.random_range(0..charlist.len());
        let c: &char = charlist.as_slice().get(x).unwrap();

        string.push(*c);

        // count the current bytes and keeps track of the target bytesize.
        if x > 128 && target_bytesize < max_size {
            target_bytesize += 1;
            bytesize += 2
        } else {
            bytesize += 1;
        }

        /* debugging stuff again. Shows password generation.*/
        if debug {
            truecount += 1;
            testing(
                string.clone(),
                x as u8,
                bytesize,
                target_bytesize,
                truecount,
            );
        }
    }
    return string;
}

/* these ones don't really need commenting they're pretty self-explanatory functions. */
pub fn intgen(length: u8, mut string: String) -> String {
    let mut random = StdRng::from_os_rng();

    for _i in 0..length {
        let x: u8 = random.random_range(48..57);
        let c: char = x.into();

        string.push(c);
    }
    return string;
}

pub fn alphanumeric(length: u8, char_min: u8, char_max: u8, mut string: String) -> String {
    for _i in 0..length {
        let mut random = StdRng::from_os_rng();
        let mut x: u8 = random.random_range(char_min..char_max);
        let c: char;

        while (58..65).contains(&x) || (91..97).contains(&x) {
            x = random.random_range(48..123);
        }

        c = x.into();
        string.push(c);
    }

    return string;
}
