## Installing:

1. Install rust from https://www.rust-lang.org/tools/install
2. Use `cargo build` to compile.
3. get the binary from `genpass-rs/target/debug/genpassrs`
4. you figure out the rest :)

Read more about the project at https://westwardfishd.me/projects/genpass-rs

## Usage

`genpassrs --help` : Prints help menu.
### String Generation
`genpassrs string --encoding extasc --length 30` : prints a string of length 30 containing random extended ascii characters, excluding spaces.
`genpassrs string --encoding ascii --spaces --length 20` : prints a string of length 20 containing random ascii characters including spaces.
### Integer Generation
`genpassrs integer --length 20` : prints a random integer of length 20. STDOUT is formatted as type: String, not integer.

# Modules
Genpassrs supports module usage outside of genpass for whatever project you are working on. Of course, you can use a wrapper to use genpassrs in any project.

## Using modules
Genpassrs includes string generation AND integer generation in stringgeneration.rs

This includes the functions: 
- generator(length:u8, char_encodingMinValue:u8, char_encodingMaxValue:u8, outputString:String) -> returns a string of specified length.
- intgen(length:u8, outputString:String) -> returns a string of integers of specified length.

This can be used in a plethora of ways-- according to your use case. 

