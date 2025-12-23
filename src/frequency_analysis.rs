/*
 * Create analytic values from strings and vectors of characters.
 */

use std::collections::HashMap;

/// Calculate the Shannon entropy of a vector of character counts.
fn shannon_entropy_vec(decomp: &Vec<usize>) -> f64 {
    let total = decomp.iter().sum::<usize>() as f64;
    return decomp
        .iter()
        .filter(|x| **x > 0)
        .map(|x| -((*x as f64) / total) * ((*x as f64) / total).log2())
        .sum();
}

/// Calculate the Shannon entropy of a string.
fn shannon_entropy_str(text: &String) -> f64 {
    let mut char_count = HashMap::new();

    /* Count the occurance of each character in the string. */
    for chr_t in text.chars() {
        char_count.entry(chr_t).and_modify(|x| *x += 1).or_insert(1);
    }
    return char_count
        .into_values()
        .map(|x| -(x as f64 / text.len() as f64) * (x as f64 / text.len() as f64).log2())
        .sum();
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

/// For a group of numbers determine the probability of encountering a specific
/// digit as the Nth digit in a number. Return an array of ten probabilities of
/// encountering each digit. Ignore the first digit in the number if it is a
/// zero.
fn digit_freq_at_idx<T: ToString>(nums: &Vec<T>, index: usize) -> Vec<f64> {
    let radix = 10;
    let mut digit_cnt = vec![0; 10];
    let mut total_digits = 0;

    /* Handle the case of no numbers. */
    if nums.is_empty() {
        return vec![0.0; 10];
    }

    /* Iterate over the numbers and convert to a string form. */
    'num_loop: for ts_num in nums {
        let mut digit_idx = 0;

        for poss_digit in ts_num.to_string().chars() {
            if poss_digit.is_digit(radix) {
                /* Find the first non-zero digit. */
                if digit_idx == 0 && poss_digit == '0' {
                    continue;
                }

                /* Collect the digit from the number at the right index. */
                if digit_idx == index && !(digit_idx == 0 && poss_digit == '0') {
                    digit_cnt[poss_digit.to_digit(radix).unwrap() as usize] += 1;
                    total_digits += 1;
                    continue 'num_loop;
                }
                digit_idx += 1;
            }
        }
    }

    /* Convert the counts to frequencies. */
    return digit_cnt
        .into_iter()
        .map(|x| (x as f64) / (total_digits as f64))
        .collect();
}

/// Calculate the absolute difference between an array of probabilities and the
/// Benford frequency of encountering that digit.
fn benford_diff(num_freq: &Vec<f64>, index: usize) -> f64 {
    return num_freq
        .into_iter()
        .enumerate()
        .map(|(digit, freq)| (freq - prob_of_benford_digit(digit, index)).abs())
        .sum::<f64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_freq_at_idx_empty_exp00() {
        assert_eq!(digit_freq_at_idx(&Vec::<u8>::new(), 0), vec![0.0; 10]);
    }

    #[test]
    fn digit_freq_at_idx_empty_exp01() {
        assert_eq!(digit_freq_at_idx(&Vec::<u8>::new(), 1), vec![0.0; 10]);
    }

    #[test]
    fn digit_freq_at_idx_empty_exp02() {
        assert_eq!(digit_freq_at_idx(&Vec::<u8>::new(), 2), vec![0.0; 10]);
    }

    #[test]
    fn digit_freq_at_idx_exp00() {
        assert_eq!(
            digit_freq_at_idx(&vec![420, 463, 981, 19, 578, 265, 39, 876, 539, 941], 0),
            vec![0.0, 0.1, 0.1, 0.1, 0.2, 0.2, 0.0, 0.0, 0.1, 0.2]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp01() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![950, -577, 261, -273, 727, -437, 781, -847, 668, -859],
                0
            ),
            vec![0.0, 0.0, 0.2, 0.0, 0.1, 0.1, 0.1, 0.2, 0.2, 0.1]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp02() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.468, 0.075, 0.850, 0.202, 0.803, 0.419, 0.473, 0.489, 0.517, 0.479
                ],
                0
            ),
            vec![0.0, 0.0, 0.1, 0.0, 0.5, 0.1, 0.0, 0.1, 0.2, 0.0]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp03() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.913, -0.473, 0.025, -0.072, 0.554, -0.072, 0.999, -0.219, 0.281, -0.302
                ],
                0
            ),
            vec![0.0, 0.0, 0.3, 0.1, 0.1, 0.1, 0.0, 0.2, 0.0, 0.2]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp04() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    -712354.191,
                    485151.655,
                    -376247.543,
                    335660.330,
                    -530630.925,
                    795.310,
                    -125823.597,
                    129188.400,
                    -41044.353,
                ],
                0
            ),
            vec![
                0.0,
                0.2222222222222222,
                0.0,
                0.2222222222222222,
                0.2222222222222222,
                0.1111111111111111,
                0.0,
                0.2222222222222222,
                0.0,
                0.0
            ]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp05() {
        assert_eq!(
            digit_freq_at_idx(&vec![420, 463, 981, 19, 578, 265, 39, 876, 539, 941], 1),
            vec![0.0, 0.0, 0.1, 0.1, 0.1, 0.0, 0.2, 0.2, 0.1, 0.2]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp06() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![950, -577, 261, -273, 727, -437, 781, -847, 668, -859],
                1
            ),
            vec![0.0, 0.0, 0.1, 0.1, 0.1, 0.2, 0.2, 0.2, 0.1, 0.0]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp07() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.468, 0.075, 0.850, 0.202, 0.803, 0.419, 0.473, 0.489, 0.517, 0.479
                ],
                1
            ),
            vec![0.2, 0.2, 0.0, 0.0, 0.0, 0.2, 0.1, 0.2, 0.1, 0.0]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp08() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.913, -0.473, 0.025, -0.072, 0.554, -0.072, 0.999, -0.219, 0.281, -0.302
                ],
                1
            ),
            vec![0.1, 0.2, 0.2, 0.0, 0.0, 0.2, 0.0, 0.1, 0.1, 0.1]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp09() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    -712354.191,
                    485151.655,
                    -376247.543,
                    335660.330,
                    -530630.925,
                    795.310,
                    -125823.597,
                    129188.400,
                    -41044.353,
                ],
                1
            ),
            vec![
                0.0,
                0.2222222222222222,
                0.2222222222222222,
                0.2222222222222222,
                0.0,
                0.0,
                0.0,
                0.1111111111111111,
                0.1111111111111111,
                0.1111111111111111,
            ]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp10() {
        assert_eq!(
            digit_freq_at_idx(&vec![420, 463, 981, 19, 578, 265, 39, 876, 539, 941], 2),
            vec![
                0.125, 0.25, 0.0, 0.125, 0.0, 0.125, 0.125, 0.0, 0.125, 0.125
            ]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp11() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![950, -577, 261, -273, 727, -437, 781, -847, 668, -859],
                2
            ),
            vec![0.1, 0.2, 0.0, 0.1, 0.0, 0.0, 0.0, 0.4, 0.1, 0.1]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp12() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.468, 0.075, 0.850, 0.202, 0.803, 0.419, 0.473, 0.489, 0.517, 0.479
                ],
                2
            ),
            vec![0.0, 0.0, 0.125, 0.25, 0.0, 0.0, 0.0, 0.125, 0.125, 0.375]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp13() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    0.913, -0.473, 0.025, -0.072, 0.554, -0.072, 0.999, -0.219, 0.281, -0.302
                ],
                2
            ),
            vec![
                0.0,
                0.14285714285714285,
                0.14285714285714285,
                0.2857142857142857,
                0.14285714285714285,
                0.0,
                0.0,
                0.0,
                0.0,
                0.2857142857142857
            ]
        )
    }

    #[test]
    fn digit_freq_at_idx_exp14() {
        assert_eq!(
            digit_freq_at_idx(
                &vec![
                    -712354.191,
                    485151.655,
                    -376247.543,
                    335660.330,
                    -530630.925,
                    795.310,
                    -125823.597,
                    129188.400,
                    -41044.353,
                ],
                2
            ),
            vec![
                0.2222222222222222,
                0.0,
                0.1111111111111111,
                0.0,
                0.0,
                0.4444444444444444,
                0.1111111111111111,
                0.0,
                0.0,
                0.1111111111111111
            ]
        )
    }

    #[test]
    fn benford_diff_exp00() {
        let val = benford_diff(
            &vec![
                0.125, 0.25, 0.0, 0.125, 0.0, 0.125, 0.125, 0.0, 0.125, 0.125,
            ],
            0,
        );

        println!("{}", val);

        assert!(0.76403 < val && val < 0.76405);
    }

    #[test]
    fn benford_diff_exp01() {
        let val = benford_diff(&vec![0.1, 0.2, 0.0, 0.1, 0.0, 0.0, 0.0, 0.4, 0.1, 0.1], 0);

        println!("{}", val);

        assert!(1.090195 < val && val < 1.090197);
    }

    #[test]
    fn benford_diff_exp02() {
        let val = benford_diff(
            &vec![0.0, 0.0, 0.125, 0.25, 0.0, 0.0, 0.0, 0.125, 0.125, 0.375],
            0,
        );

        println!("{}", val);
        assert!(1.19030 < val && val < 1.19032);
    }

    #[test]
    fn benford_diff_exp03() {
        let val = benford_diff(
            &vec![
                0.0,
                0.14285714285714285,
                0.14285714285714285,
                0.2857142857142857,
                0.14285714285714285,
                0.0,
                0.0,
                0.0,
                0.0,
                0.2857142857142857,
            ],
            0,
        );

        println!("{}", val);
        assert!(0.89335893 < val && val < 0.89335895);
    }

    #[test]
    fn benford_diff_exp04() {
        let val = benford_diff(
            &vec![
                0.2222222222222222,
                0.0,
                0.1111111111111111,
                0.0,
                0.0,
                0.4444444444444444,
                0.1111111111111111,
                0.0,
                0.0,
                0.1111111111111111,
            ],
            0,
        );

        println!("{}", val);
        assert!(1.394005 < val && val < 1.394007);
    }

    #[test]
    fn benford_diff_exp05() {
        let val = benford_diff(
            &vec![
                0.125, 0.25, 0.0, 0.125, 0.0, 0.125, 0.125, 0.0, 0.125, 0.125,
            ],
            1,
        );

        println!("{}", val);
        assert!(0.598963380 < val && val < 0.598963382);
    }

    #[test]
    fn benford_diff_exp06() {
        let val = benford_diff(&vec![0.1, 0.2, 0.0, 0.1, 0.0, 0.0, 0.0, 0.4, 0.1, 0.1], 1);

        println!("{}", val);
        assert!(0.846381002 < val && val < 0.846381004);
    }

    #[test]
    fn benford_diff_exp07() {
        let val = benford_diff(
            &vec![0.0, 0.0, 0.125, 0.25, 0.0, 0.0, 0.0, 0.125, 0.125, 0.375],
            1,
        );

        println!("{}", val);
        assert!(1.047859090 < val && val < 1.047859092);
    }

    #[test]
    fn benford_diff_exp08() {
        let val = benford_diff(
            &vec![
                0.0,
                0.14285714285714285,
                0.14285714285714285,
                0.2857142857142857,
                0.14285714285714285,
                0.0,
                0.0,
                0.0,
                0.0,
                0.2857142857142857,
            ],
            1,
        );
        assert!(0.9753064 < val && val < 0.9753066);
    }

    #[test]
    fn benford_diff_exp09() {
        let val = benford_diff(
            &vec![
                0.2222222222222222,
                0.0,
                0.1111111111111111,
                0.0,
                0.0,
                0.4444444444444444,
                0.1111111111111111,
                0.0,
                0.0,
                0.1111111111111111,
            ],
            1,
        );
        assert!(0.99289980 < val && val < 0.99289982);
    }

    #[test]
    fn benford_diff_exp10() {
        let val = benford_diff(
            &vec![
                0.125, 0.25, 0.0, 0.125, 0.0, 0.125, 0.125, 0.0, 0.125, 0.125,
            ],
            2,
        );
        assert!(0.60032 < val && val < 0.60034);
    }

    #[test]
    fn benford_diff_exp11() {
        let val = benford_diff(&vec![0.1, 0.2, 0.0, 0.1, 0.0, 0.0, 0.0, 0.4, 0.1, 0.1], 2);
        assert!(0.8052 < val && val < 0.8054);
    }

    #[test]
    fn benford_diff_exp12() {
        let val = benford_diff(
            &vec![0.0, 0.0, 0.125, 0.25, 0.0, 0.0, 0.0, 0.125, 0.125, 0.375],
            2,
        );
        assert!(1.004 < val && val < 1.006);
    }

    #[test]
    fn benford_diff_exp13() {
        let val = benford_diff(
            &vec![
                0.0,
                0.14285714285714285,
                0.14285714285714285,
                0.2857142857142857,
                0.14285714285714285,
                0.0,
                0.0,
                0.0,
                0.0,
                0.2857142857142857,
            ],
            2,
        );
        assert!(0.9971 < val && val < 0.9973);
    }

    #[test]
    fn benford_diff_exp14() {
        let val = benford_diff(
            &vec![
                0.2222222222222222,
                0.0,
                0.1111111111111111,
                0.0,
                0.0,
                0.4444444444444444,
                0.1111111111111111,
                0.0,
                0.0,
                0.1111111111111111,
            ],
            2,
        );
        assert!(0.9994 < val && val < 0.9996);
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
        assert_eq!(shannon_entropy_vec(&vec![1]), 0.0);
    }

    #[test]
    fn shannon_entropy_vec_exp01() {
        let result = shannon_entropy_vec(&vec![1, 2, 0, 0, 12, 23, 2]);
        assert!(1.544 < result && result < 1.546);
    }

    #[test]
    fn shannon_entropy_vec_exp02() {
        let result = shannon_entropy_vec(&vec![1, 2, 1, 1, 2, 3, 2, 1, 1, 1, 1, 1]);
        assert!(3.453 < result && result < 3.455);
    }

    #[test]
    fn shannon_entropy_vec_exp03() {
        assert_eq!(shannon_entropy_vec(&vec![0]), 0.0);
    }

    #[test]
    fn shannon_entropy_vec_exp04() {
        let result = shannon_entropy_vec(&vec![40, 5656, 775, 55, 1, 693, 78, 7332, 45, 6]);
        assert!(1.586 < result && result < 1.588);
    }

    #[test]
    fn shannon_entropy_vec_empty() {
        assert_eq!(shannon_entropy_vec(&Vec::new()), 0.0);
    }

    #[test]
    fn shannon_entropy_str_exp00() {
        let result = shannon_entropy_str(&String::from("AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe"));
        assert!(4.649 < result && result < 4.651);
    }

    #[test]
    fn shannon_entropy_str_exp01() {
        let result = shannon_entropy_str(&String::from(
            "An ounce of prevention is worth a pound of cure.",
        ));
        assert!(3.851 < result && result < 3.853);
    }

    #[test]
    fn shannon_entropy_str_exp02() {
        let result = shannon_entropy_str(&String::from(
            "Humans are the weakest link in any security chain.",
        ));
        assert!(3.876 < result && result < 3.878);
    }

    #[test]
    fn shannon_entropy_str_exp03() {
        assert_eq!(shannon_entropy_str(&String::from("abcdefghijklmnop")), 4.0);
    }

    #[test]
    fn shannon_entropy_str_exp04() {
        let result = shannon_entropy_str(&String::from(concat!(
            "看官，現今我們中國四萬萬同胞欲內免專制、外杜瓜分的一個絕大轉機、絕大遭際，不",
            "是那預備立憲一事麼？但那立憲上加了這麼預備兩個字的活動考語，我就深恐將來這瘟",
            "憲立不成，必定嫁禍到我們同胞程度不齊上，以為卸罪地步。唉！說也可憐，卻難怪政",
            "府這般設想，中國人卻也真沒得立憲國民的資格。語云：「物必自腐而後蟲生，人必自",
            "侮而後人侮之。」所以無論強弱榮辱，皆是自己做出來的，切莫要去錯怨別人。看官，",
            "你們如果不信我們中國社會腐敗沒有立憲國文明的氣象，我曾經得著一部社會小說，其",
            "中類皆近世實人實事，怪怪奇奇，莫可名狀，足能做一本立憲難成的保證書。我若不從",
            "頭至尾的細細說明，不獨看官們裝在一個大悶葫蘆裡頭疑團莫釋，連我也未免辜負那贈",
            "書的人一番苦心孤詣"
        )));
        assert!(2.900 < result && result < 2.902);
    }

    #[test]
    fn shannon_entropy_str_empty() {
        assert_eq!(shannon_entropy_str(&String::from("")), 0.0);
    }
}
