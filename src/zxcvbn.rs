use zxcvbn::{zxcvbn, Score};

pub fn estimate(string: String) -> Score {
    let estimatescore = zxcvbn(&string, &[]);
    return estimatescore.score();
}
