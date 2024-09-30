## Installing:

### For Windows Users:

There is now a precompiled binary attached to all releases <= 1.0.2
You can simply run the .exe in a command prompt to use the software as desired.

The binary was compiled using a x86_64 bit processor, so be warned!

### Manual Installation:

1. Install rust from https://www.rust-lang.org/tools/install
2. Use `cargo build --release` to compile.
3. get the binary from `genpass-rs/target/build/genpassrs`
4. you figure out the rest :)

Read more about the project at https://westwardfishd.me/projects/genpass-rs

## Usage

`genpassrs --help` : Prints help menu.

### String Generation

`genpassrs string --encoding extasc --length 30` : prints a string of length 30 containing random extended ascii characters, excluding spaces.

`genpassrs string --encoding ascii --spaces --length 20` : prints a string of length 20 containing random ascii characters including spaces.

`genpassrs alphanumeric -l 25` : generates an alphanumeric string of length 25. 

`genpassrs alphanumeric -a 25` : generates a string of length 25 of only letters of varying cases.

`genpassrs alphanumeric -s -l 25` : generates a string of length 25 with lowercase letters.

`genpassrs alphanumeric -u -l 25` : generates a string of length 25 with uppercase letters.

### Integer Generation

`genpassrs integer --length 20` : prints a random integer of length 20. STDOUT is formatted as type: String, not integer.

### Password Strength Estimation:

`genpassrs estimate <string>` OR `<stdin> | genpassrs estimate -`

# Modules

Genpassrs supports module usage outside of genpass for whatever project you are working on. Of course, you can use a wrapper to use genpassrs in any project.

## Using modules
Genpassrs includes string generation AND integer generation in stringgeneration.rs

This includes the functions: 
- generator(length:u8, char_encodingMinValue:u8, char_encodingMaxValue:u8, outputString:String) -> returns a string of specified length.
- intgen(length:u8, outputString:String) -> returns a string of integers of specified length.
- alphanumeric(length:u8, char_min:u8, char_max:u8, outputString:String)-> returns a string of alphanumeric characters.

This can be used in a plethora of ways-- according to your use case. 

