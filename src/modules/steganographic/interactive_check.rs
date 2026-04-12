use zxcvbn::zxcvbn;
pub fn interactively_check_password(password: &str) -> bool {
    //! Checks the passkey strength of a given password interactively--
    //! if the password strength is not strong enough; the program will
    //! reask for the password (for steganography, may move there.)
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
