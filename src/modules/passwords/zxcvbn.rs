use zxcvbn::zxcvbn;

#[doc(hidden)]
pub fn estimate(string: String) {
    //! estimates the given strength of the password and prints a nice message.
    let estimatescore = zxcvbn(&string, &[]);
    let score = u8::from(estimatescore.score());

    print!("zxcvbn score for '{}': {}\n", &string, score);
    match score{
        0 => println!(
            "This password is extremely weak, as it would take at least 100 guesses to crack."
        ),
        1 => println!(
            "This password is moderately weak, as it would take at least 100,000 guesses to crack."
        ),
        2 => println!(
            "This password is slightly strong, as it would take at least 100,000,000 guesses to crack."
        ),
        3 => println!(
            "This password is strong, as it would take at least 10,000,000,000 guesses to crack."
        ),
        4 => println!(
            "This password is extremely strong, as it would take more than 10,000,000,000 guesses to crack."
        ),
        _ => ()
    }
}
