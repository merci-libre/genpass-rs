
# Merci-Libre's Genpass-rs
The official page for Merci-Libre's genpass-rs. Created by [westwardfishdme](https://github.com/westwardfishdme).
## Installing:

### For Windows Users:

There is now a precompiled binary attached to all releases <= 1.0.2
You can simply run the .exe in a command prompt to use the software as desired.

The binary was compiled using for x86_64 systems. Can be obtained in the [releases tab](https://github.com/merci-libre/genpass-rs/releases)

### Linux:

There is also a binary for x86_64 systems. inside of the [releases tab](https://github.com/merci-libre/genpass-rs/releases) :)

### From Source:

1. Install rust from https://www.rust-lang.org/tools/install
2. Use `cargo build --release` to compile.
3. get the binary from `genpass-rs/target/build/genpassrs`
4. you figure out the rest :)

Read more about the project on my [website](https://westwardfishdme.github.io)

## Usage

`genpassrs --help` : Prints help menu.

### String Generation

`genpassrs string --encoding extasc --length 30` : prints a string of length 30 containing random extended ascii characters, excluding spaces.

`genpassrs string --encoding ascii --spaces --length 20` : prints a string of length 20 containing random ascii characters including spaces.

`genpassrs alphanumeric -l 25` : generates an alphanumeric string of length 25. 

`genpassrs alphanumeric -a -l 25` : generates a string of length 25 of only letters of varying cases.

`genpassrs alphanumeric -s -l 25` : generates a string of length 25 with lowercase letters.

`genpassrs alphanumeric -u -l 25` : generates a string of length 25 with uppercase letters.

### Integer Generation

`genpassrs integer --length 20` : prints a random integer of length 20. STDOUT is formatted as type: String, not integer.

### Password Strength Estimation:

`genpassrs estimate <string>` OR `<stdin> | genpassrs estimate -`
## Using Steganographic Functions to embed or store passwords/messages.

This program uses 2 crates for steganographic functionality: 
- [Stegano](https://github.com/wiseaidev/stegano) for encrypting and formatting payloads.
- [Steganography](https://github.com/teovoinea/steganography) for actually embedding the payloads into the images.

When encrypting a payload into an image, Genpass-rs uses AES-128 to securely store up to 240-byte long strings into images.
To use the steganographic functions the inputted file must meet the following criteria:
- Must be a [.jpg, .jpeg, or .png]
- Must be at least 1kb in size, however as of 1.1.2 there is no check on file size. Meaning that smaller images may result in an error or crash.
- Must not have any previous data embedded into an image using this program (or other programs using steganography), there is no way to check for this in the software at the current moment, so use fresh unedited images before using this command!

The steganographic file will be outputted as a `.png`

### Storing and Reading
Genpass-rs comes with 2 methods of storing passwords into images:
1. Generate a password up to 240 bytes (240 characters, or 120 characters with utf-8 converted extended ascii),
2. Storing a payload up to 240 bytes.
You may choose to encrypt the payloads before storing them into images by passing the `-u` argument.

For example:

`genpassrs steg embed -n some.png -p "Hello World" -u` 

This command will store the payload `Hello World` into the image `some.png` without encryption.   

For generating a new random password with encryption, you can use the following command:

`genpassrs steg generate -n some.jpeg -l 50 -e extasc -s`

This command will generate a new passphrase with both ascii and extended ascii of length `50`, and then store it into the file `some.jpeg`.  

# Modules

~~Genpassrs supports module usage outside of genpass for whatever project you are working on. Of course, you can use a wrapper to use genpassrs in any project.~~
(this is currently broken in the current patch-- will future updates will make modules available again soon.)

