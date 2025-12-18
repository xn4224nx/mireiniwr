/*
 * Create analytic values from strings and vectors of characters.
 */

/// Calculate the Shannon entropy of a vector of character counts.
fn shannon_entropy_vec(decomp: &Vec<char>) -> f64 {
    0
}

/// Calculate the Shannon entropy of a string.
fn shannon_entropy_str(text: &String) -> f64 {
    0
}

/// According to Benford's law what is the probability of finding a specific
/// digit at a specific point in a number.
fn prob_of_benford_digit(digit: usize, position: usize) -> f64 {
    let probs = vec![
        vec![
            0.0,
            0.3010299956639812,
            0.17609125905568124,
            0.12493873660829992,
            0.09691001300805642,
            0.07918124604762482,
            0.06694678963061322,
            0.05799194697768673,
            0.05115252244738129,
            0.04575749056067514,
        ],
        vec![
            0.11967926859688073,
            0.1138901034075564,
            0.10882149900550823,
            0.10432956023095939,
            0.10030820226757937,
            0.09667723580232243,
            0.09337473578303615,
            0.09035198926960332,
            0.08757005357886138,
            0.08499735205769224,
        ],
        vec![
            0.1017843646442167,
            0.10137597744780127,
            0.10097219813704165,
            0.1005729321109262,
            0.1001780876279476,
            0.09978757569217742,
            0.09940130994496177,
            0.09901920656189599,
            0.09864118415477721,
            0.09826716367825329,
        ],
        vec![
            0.10017614693993555,
            0.100136888117578,
            0.1000976725946149,
            0.10005850028348687,
            0.10001937109690452,
            0.09998028494784099,
            0.09994124174952602,
            0.09990224141544911,
            0.09986328385937243,
            0.09982436899529125,
        ],
        vec![
            0.100017591505929,
            0.10001368113544618,
            0.10000977119522403,
            0.1000058616851637,
            0.1000019526051873,
            0.09999804395520129,
            0.09999413573512496,
            0.09999022794487125,
            0.09998632058435514,
            0.09998241365348551,
        ],
    ];

    /* Ensure the array access is valid.*/
    return if digit > 9 || position >= probs.len() {
        0.1
    } else {
        probs[position][digit]
    };
}

/// For a group of numbers work out the difference between the actual frequency
/// of each starting digit compared to its frequency predicted by Benford's
/// law. Return the sum of the absolute differences.
fn benford_first_digit_diff(nums: &Vec<T>) -> f64 {
    0
}

/// For a group of numbers work out the difference between the actual frequency
/// of each digit in the first three positions compared to its frequency
/// predicted by Benford's law. Return the sum of the absolute differences.
fn benford_three_digit_diff(nums: &Vec<T>) -> f64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benford_first_digit_diff_exp00() {
        assert_eq!(benford_first_digit_diff(&vec![0]), 0)
    }

    #[test]
    fn benford_first_digit_diff_exp01() {
        assert_eq!(benford_first_digit_diff(&vec![0]), 0)
    }

    #[test]
    fn benford_first_digit_diff_exp02() {
        assert_eq!(benford_first_digit_diff(&vec![0]), 0)
    }

    #[test]
    fn benford_first_digit_diff_exp03() {
        assert_eq!(benford_first_digit_diff(&vec![0]), 0)
    }

    #[test]
    fn benford_first_digit_diff_exp04() {
        assert_eq!(benford_first_digit_diff(&vec![0]), 0)
    }

    #[test]
    fn prob_of_benford_digit_extreme_exp00() {
        assert_eq!(prob_of_benford_digit(10, 0), 0.1);
    }

    #[test]
    fn prob_of_benford_digit_extreme_exp01() {
        assert_eq!(prob_of_benford_digit(3, 5), 0.1);
    }

    #[test]
    fn prob_of_benford_digit_extreme_exp02() {
        assert_eq!(prob_of_benford_digit(10, 12), 0.1);
    }

    #[test]
    fn shannon_entropy_vec_exp00() {
        assert_eq!(shannon_entropy_vec(&vec![1]), 0);
    }

    #[test]
    fn shannon_entropy_vec_exp01() {
        assert_eq!(
            shannon_entropy_vec(&vec![1, 2, 0, 0, 12, 23, 2]),
            1.5453912199382331
        );
    }

    #[test]
    fn shannon_entropy_vec_exp02() {
        assert_eq!(
            shannon_entropy_vec(&vec![1, 2, 1, 1, 2, 3, 2, 1, 1, 1, 1, 1]),
            3.4548223999466066
        );
    }

    #[test]
    fn shannon_entropy_vec_exp03() {
        assert_eq!(shannon_entropy_vec(&vec![0]), 0);
    }

    #[test]
    fn shannon_entropy_vec_exp04() {
        assert_eq!(
            shannon_entropy_vec(&vec![40, 5656, 775, 55, 1, 693, 78, 7332, 45, 6]),
            1.5870514669732283
        );
    }

    #[test]
    fn shannon_entropy_vec_empty() {
        assert_eq!(shannon_entropy_vec(&Vec::new()), 0);
    }

    #[test]
    fn shannon_entropy_str_exp00() {
        assert_eq!(
            shannon_entropy_str(&String::from("AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe")),
            4.65066
        );
    }

    #[test]
    fn shannon_entropy_str_exp01() {
        assert_eq!(
            shannon_entropy_str(&String::from(
                "An ounce of prevention is worth a pound of cure."
            )),
            3.8524934279
        );
    }

    #[test]
    fn shannon_entropy_str_exp02() {
        assert_eq!(
            shannon_entropy_str(&String::from(
                "Humans are the weakest link in any security chain."
            )),
            3.8770822612
        );
    }

    #[test]
    fn shannon_entropy_str_exp03() {
        assert_eq!(shannon_entropy_str(&String::from("abcdefghijklmnop")), 4.0);
    }

    #[test]
    fn shannon_entropy_str_exp04() {
        assert_eq!(
            shannon_entropy_str(&String::from(concat!(
                "看官，現今我們中國四萬萬同胞欲內免專制、外杜瓜分的一個絕大轉機、絕大遭際，不",
                "是那預備立憲一事麼？但那立憲上加了這麼預備兩個字的活動考語，我就深恐將來這瘟",
                "憲立不成，必定嫁禍到我們同胞程度不齊上，以為卸罪地步。唉！說也可憐，卻難怪政",
                "府這般設想，中國人卻也真沒得立憲國民的資格。語云：「物必自腐而後蟲生，人必自",
                "侮而後人侮之。」所以無論強弱榮辱，皆是自己做出來的，切莫要去錯怨別人。看官，",
                "你們如果不信我們中國社會腐敗沒有立憲國文明的氣象，我曾經得著一部社會小說，其",
                "中類皆近世實人實事，怪怪奇奇，莫可名狀，足能做一本立憲難成的保證書。我若不從",
                "頭至尾的細細說明，不獨看官們裝在一個大悶葫蘆裡頭疑團莫釋，連我也未免辜負那贈",
                "書的人一番苦心孤詣"
            ))),
            7.1202214637
        );
    }

    #[test]
    fn shannon_entropy_str_empty() {
        assert_eq!(shannon_entropy_str(&String::from("")), 0);
    }
}
