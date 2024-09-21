/*use this when debugging.*/
//use std::{thread};

use rand::Rng;

pub fn generator(length: u8, char_min: u8, char_max: u8, mut string: String) -> String {
    let mut previous_x: u8;
    previous_x = 0;

    let mut bytesize: i16 = 0;
    let mut target_bytesize: i16 = length.into();
    let max_size: i16 = target_bytesize * 2;

    /*use this when debugging.*/
    // let mut truecount = 0;

    while bytesize != target_bytesize {
        let mut random = rand::thread_rng();
        let mut x: u8 = random.gen_range(char_min..char_max);
        let c: char;

        //remove unused table portions and control characters.
        while (127..161).contains(&x) {
            x = random.gen_range(char_min..char_max);
        }
        while x == previous_x {
            x = random.gen_range(char_min..char_max);
        }
        c = x.into();
        string.push(c);

        if x > 128 && target_bytesize < max_size {
            target_bytesize = target_bytesize + 1;
            bytesize += 2
        } else {
            bytesize += 1;
        }
        /* code block I use for debugging and ensuring the byte size of string is correct
        truecount += 1;
        println!("\nstring: {string}");
        println!("current_bytesize={bytesize}");
        println!("target_bytesize={target_bytesize}");
        println!("character value: {x} ");
        println!("truecount={truecount}");
        thread::sleep(time::Duration::from_millis(100));
        */
        previous_x = x;
    }
    return string;
}