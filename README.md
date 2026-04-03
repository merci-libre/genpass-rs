# Genpass-rs
The official page for Merci-Libre's genpass-rs. Created by [westwardfishdme](https://github.com/westwardfishdme). Now v2.0!

- [Installation](#Installing)
  - [Windows](#Windows)
  - [Linux](#Linux)
  - [From Source](#Building-From-Source)
- [Usage](#Usage)
- [Thanks](#Thanks)


## Installing
Installing genpass-rs is not supported via package managers yet. All x86_64 builds can be obtained from the 
[releases tab](https://github.com/merci-libre/genpass-rs/releases).

### Windows

There is now a precompiled binary attached to all releases <= 1.0.2
You can simply run the .exe in a command prompt to use the software as desired.

The binary was compiled using for x86_64 systems. Can be obtained in the [releases tab](https://github.com/merci-libre/genpass-rs/releases).

### Linux
There is a pre-compiled binary for x86_64 systems. Can also be obtained from the [releases tab](https://github.com/merci-libre/genpass-rs/releases). :)

### Building From Source

1. Install rust from https://www.rust-lang.org/tools/install
2. Use `cargo build --release` to compile.
3. get the binary from `genpass-rs/target/build/genpassrs`
4. you figure out the rest :)

## Usage
Genpass-rs supports up to string generation up to 255 characters. This was explicitly chosen as most password fields are limited to 32 characters,
albeit the maximum supported amount of characters is 255. This limit is currently not bypassable, and is enforced for all forms of passphrase generation.

### Getting Help

`genpassrs --help` : Prints help menu, in addition since this project uses clap you can see other commands with the following syntax:

```
genpassrs <subcommand> --help
```

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
- Must be a [.jpg, .jpeg, or .png], this is checked at run-time by checking the file's magic bytes.
- Must be at least 1kb in size, however as of 1.1.2 there is no check on file size. Meaning that smaller images may result in undefined behavior.
- Must not have any previous data embedded into an image using this program (or other programs using steganography), there is no way to check for this 
in the software at the current moment, so use fresh unedited images before using this command!

The steganographic file will be outputted as a `.png`

### Storing and Reading
Genpass-rs comes with 2 methods of storing passwords into images:
1. Generate a password.
2. Or storing a payload.

You may choose to leave payloads unencrypted before storing them into images by passing the `-u` argument. For example:

`genpassrs steg embed -n some.png -p "Hello World" -u` 

This command will store the payload `Hello World` into the image `some.png` without encryption.   

### Generation

For generating a new random password with encryption, you can use the following command:

`genpassrs steg generate -n some.jpeg -l 50 -e extasc -s`

This command will generate a new passphrase with both ascii and extended ascii of length `50`, and then store it into the file `some.jpeg`.  

# Modules

~~Genpassrs supports module usage outside of genpass for whatever project you are working on. Of course, you can use a wrapper to use genpassrs in any project.~~
(this is currently broken in the current patch-- will future updates will make modules available again soon.)

# Thanks

First and foremost thank you for checking out this repository. As the developer of this project I had spent 2 years perfecting this project, and improving it.
With that said, I want to give a huge thanks to users of this program, and the developers of the projects that had help make this project possible.
