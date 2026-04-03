use console;
use std::{fs::File, io::Read};

use zxcvbn::zxcvbn;

#[doc(hidden)]
pub fn estimate(string: String) {
    //! estimates the given strength of the password and prints a nice message.
    let estimatescore = zxcvbn(&string, &[]);
    let score = u8::from(estimatescore.score());
    let guesses = estimatescore.guesses();

    print!("zxcvbn score for '{}': {}\n", &string, score);
    match score{
        0 => println!(
            "This password is extremely weak, as it would take at least {guesses} guesses to crack."
        ),
        1 => println!(
            "This password is moderately weak, as it would take at least {guesses} guesses to crack.",
        ),
        2 => println!(
            "This password is slightly strong, as it would take at least {guesses} guesses to crack.",
        ),
        3 => println!(
            "This password is strong, as it would take at least {guesses} guesses to crack."
        ),
        4 => println!(
            "This password is extremely strong, as it would take more than {guesses} guesses to crack."
        ),
        _ => ()
    }
}

pub fn check_password_strength(password: &str) -> bool {
    let term = console::Term::stderr();
    let estimatescore = zxcvbn(password, &[]);
    let score = u8::from(estimatescore.score());
    let guess_amount = estimatescore.guesses();
    let reask_password = false;

    if score < 3 {
        eprint!(
            "Warning: This password is weak according to zxcvbn [crackable in {guess_amount} guesses]\nAre you sure you want to use this as your password? [y/n]: "
        );
        let yes_or_no = match term.read_char() {
            Ok(v) => v,
            Err(_) => std::process::exit(1),
        };

        return match yes_or_no {
            'n' | 'N' => true,
            'y' | 'Y' => false,
            _ => true,
        };
    }
    reask_password
}
