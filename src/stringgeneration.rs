use rand::Rng;
/*
* [stringgeneration.rs]
*
* This is the password engine used in genpass-rs. It uses the CSPRNG of the rand
* crate to generate randomized strings. It supports all UTF-8 character generation
* up to 2^8 (0-255).
*
* There are seperate functions for separate needs, for example-- the first function below: `fn generator()`
* specifically creates passwords with the highest amount of entropy that this program can allow. It
* does this in a atypical manner.
*
* You see, the biggest issue I had when originally trying to figure out how to manage characters in
* the range >127 was that they account for 2 bytes, i.e. char: 175 = 2 bytes, while char: 124 = 1 byte).
*
* As you can imagine, that can be pretty frustrating if you're having to check if a string's length
* is at '15' because String::length() calculates the bytes within an array.
*
* */

use std::{io, thread, time};
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
    length: u8,
    char_min: u8,
    char_max: u8,
    mut string: String,
    debug: bool,
) -> String {
    let mut bytesize: i16 = 0;
    let mut target_bytesize: i16 = length.into();
    let max_size: i16 = target_bytesize * 2;
    let mut truecount: u16 = 0;

    /* [about the variables]
     *
     * bytesize:i16= accounts for the current amount of bytes the program successfully generates.
     * target_bytesize: the length that the user had specified.
     * max_size: keeps the actual target value in bytes.
     *
     *
     * max_size needs to always be twice the size of the target bytes, because a single character
     * can account for 2 bytes, so if I only was to generate 15 characters of characters above
     * char: 127, the maximum size of the buffer can only be 30 bytes.
     *
     * You'll see how the logic works in the `while` loop below.
     *
     * */

    while bytesize != target_bytesize {
        // continuously generates random characters until the target_bytesize is reached.
        let mut random = rand::thread_rng();
        let mut x: u8 = random.gen_range(char_min..char_max);
        let c: char;

        //if the character generated is an escape code, or unused table entry: regenerate.
        while (127..161).contains(&x) {
            x = random.gen_range(char_min..char_max);
        }
        c = x.into();
        string.push(c);

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
            testing(string.clone(), x, bytesize, target_bytesize, truecount);
        }
    }
    return string;
}

/* these ones don't really need commenting they're pretty self-explanatory functions. */
pub fn intgen(length: u8, mut string: String) -> String {
    let mut random = rand::thread_rng();

    for _i in 0..length {
        let x: u8 = random.gen_range(48..57);
        let c: char = x.into();

        string.push(c);
    }
    return string;
}

pub fn alphanumeric(length: u8, char_min: u8, char_max: u8, mut string: String) -> String {
    for _i in 0..length {
        let mut random = rand::thread_rng();
        let mut x: u8 = random.gen_range(char_min..char_max);
        let c: char;

        while (58..65).contains(&x) || (91..97).contains(&x) {
            x = random.gen_range(48..123);
        }

        c = x.into();
        string.push(c);
    }

    return string;
}
