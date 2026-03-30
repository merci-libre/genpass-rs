use std::{error::Error, fs, process::exit};

pub fn check_magic(file: &String) -> Result<bool, Box<dyn Error>> {
    // Checks the magic bytes of the inputted file to ensure that
    // the image file is supported.
    let file = fs::read(file)?;
    let start = [file[0], file[1], file[2], file[3]];
    // Supports .jpg, .jpeg, and .png

    match start {
        [0xff, 0xd8, _, _] => (),       //jpg, jpeg
        [0x89, 0x50, 0x4e, 0x47] => (), // PNG
        [_, _, _, _] => {
            eprintln!("Error: File is not a jpg, jpeg, or PNG");
            exit(1)
        }
    }
    Ok(true)
}
