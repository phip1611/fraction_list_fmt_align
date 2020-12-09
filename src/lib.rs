/*
MIT License

Copyright (c) 2020 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! Formats a list of arbitrary fractional numbers (either string or f32/f64) so that they are
//! correctly aligned when printed line by line. It also removes unnecessary zeroes. This means
//! that it rather returns "7" instead of "7.000000".
//!
//! ## Example Input
//! ```text
//! "-42"
//! "0.3214"
//! "1000"
//! "-1000.2"
//! ```
//! ## Example Output
//! ```text
//! "  -42"
//! "    0.3214"
//! " 1000"
//! "-1000.2"

use regex::Regex;

/// Internal structure that holds the parts of a fraction number string.
/// Can represent e.g. "1", "3.14", or "-14.141".
/// The fraction part is already optimized, i.e. Some("123000") becomes ("123"),
/// "00000" becomes None etc.
struct FractionNumberParts {
    // if it has "-" sign
    has_sign: bool,
    whole_part: String,
    fraction_part: Option<String>,
}

/// Splits a formatted fraction number into its parts.
/// We expect only valid values at this point.
fn get_fraction_number_parts(formatted_fraction_num: &str) -> FractionNumberParts {
    let regex =
        Regex::new("^(?P<sign>-)?(?P<whole_part>[0-9]+)(.(?P<fraction_part>[0-9]+))?$").unwrap();
    let captures = regex.captures(formatted_fraction_num).unwrap();

    let fraction_part = captures
        .name("fraction_part")
        .map(|x| x.as_str().to_owned());
    let fraction_part = normalized_fraction_part_or_none(fraction_part);

    FractionNumberParts {
        has_sign: captures.name("sign").is_some(),
        whole_part: captures
            .name("whole_part")
            .map(|x| x.as_str().to_owned())
            .unwrap(),
        fraction_part,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FractionNumber {
    F32(f32),
    F64(f64),
}

impl From<f32> for FractionNumber {
    fn from(val: f32) -> Self {
        FractionNumber::F32(val)
    }
}

impl From<f64> for FractionNumber {
    fn from(val: f64) -> Self {
        FractionNumber::F64(val)
    }
}

impl FractionNumber {
    fn format(self, precision: u8) -> String {
        match self {
            FractionNumber::F32(val) => {
                format!("{:.1$}", val, precision as usize)
            }
            FractionNumber::F64(val) => {
                format!("{:.1$}", val, precision as usize)
            }
        }
    }
}

/// Convenient wrapper around [`fmt_align_fraction_strings`] that takes
/// a slice of floating point values, formats them all with a maximum
/// precision and returns a list of aligned, formatted strings.
/// * `max_precision` Maximum precision. This means for example if `max_precision`
///                   is 2 than "0.123" will become "0.12" whereas "2.000" will become "2".
pub fn fmt_align_fractions(fractions: &[FractionNumber], max_precision: u8) -> Vec<String> {
    let fraction_strings = fractions
        .iter()
        .map(|fr| fr.format(max_precision))
        .collect::<Vec<String>>();

    let str_vec = fraction_strings
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();

    fmt_align_fraction_strings(&str_vec)
}

/// Aligns a number of formatted fraction numbers. Valid strings are for example
/// `1`, `3.14`, and `-42`. Aligns all with additional padding on the left so that
/// all of them can be printed line by line in an aligned way. This means that
/// in every line the tens digits will be aligned, the once places will be aligned,
/// the decimal place will be aligned etc. (TODO are these the proper english terms?)
/// ## Example Input
/// ```text
/// "-42"
/// "0.3214"
/// "1000"
/// "-1000.2"
/// ```
/// ## Example Output
/// ```text
/// "  -42"
/// "    0.3214"
/// " 1000"
/// "-1000.2"
/// ```
pub fn fmt_align_fraction_strings(strings: &[&str]) -> Vec<String> {
    let fract_parts = strings
        .iter()
        .map(|x| get_fraction_number_parts(*x))
        .collect::<Vec<FractionNumberParts>>();

    // the new formatted vectors
    let mut new_formatted: Vec<String> = vec![String::new(); fract_parts.len()];

    let max_digits_whole_part = fract_parts
        .iter()
        .map(|x| x.whole_part.len())
        .max()
        .unwrap();
    let max_digits_whole_part_with_sign = fract_parts
        .iter()
        .filter(|x| x.whole_part.len() == max_digits_whole_part)
        .any(|x| x.has_sign);

    fract_parts.iter().enumerate().for_each(|(index, f)| {
        let max_len_whole_part_including_sign = if max_digits_whole_part_with_sign {
            max_digits_whole_part + 1
        } else {
            max_digits_whole_part
        };

        let current_len = f.whole_part.len() + if f.has_sign { 1 } else { 0 };

        let spaces = max_len_whole_part_including_sign - current_len;

        let mut regular_format = f.whole_part.to_string();
        if let Some(ref val) = f.fraction_part {
            regular_format = format!("{}.{}", regular_format, val);
        }
        if f.has_sign {
            regular_format = format!("-{}", regular_format);
        }

        let mut spaces_fmt = String::new();
        for _ in 0..spaces {
            spaces_fmt.push(' ');
        }

        new_formatted[index] = format!("{}{}", spaces_fmt, regular_format);
    });

    new_formatted
}

/// Takes the fraction part, removes all zeroes and afterwards returns
/// the new fraction part string. If after the removing of the zeroes
/// only "" is left, than None get's returned.
fn normalized_fraction_part_or_none(mut fp: Option<String>) -> Option<String> {
    fp = fp.map(|x| fraction_part_remove_zeroes(&x));
    if fp.is_some() && fp.as_ref().unwrap().is_empty() {
        None
    } else {
        fp
    }
}

/// Uses [`fraction_part_count_zeroes`] to remove unnecessary zeroes in a fraction string
/// (only the fraction part).
fn fraction_part_remove_zeroes(fraction_part: &str) -> String {
    let zeroes = fraction_part_count_zeroes(fraction_part);
    let slice = &fraction_part[0..fraction_part.len() - zeroes];
    slice.to_string()
}

/// Tells that in "123000" (fraction part of "0.123000") are three unnecessary zeroes.
fn fraction_part_count_zeroes(fraction_part: &str) -> usize {
    let mut zeroes = 0;
    let chars = fraction_part.chars().collect::<Vec<char>>();
    for i in 0..fraction_part.len() {
        // go backwards
        let i = fraction_part.len() - 1 - i;
        let char = chars[i];
        if char == '0' {
            zeroes += 1;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_fraction_number_parts() {
        let num = -13.37_f64;
        let num_fmt = format!("{:.2}", num);
        let parts = get_fraction_number_parts(&num_fmt);
        assert_eq!(true, parts.has_sign);
        assert_eq!("13", parts.whole_part);
        assert_eq!("37", parts.fraction_part.unwrap());

        let num = 1411010_f64;
        let num_fmt = format!("{}", num);
        let parts = get_fraction_number_parts(&num_fmt);
        assert_eq!(false, parts.has_sign);
        assert_eq!("1411010", parts.whole_part);
        assert_eq!(true, parts.fraction_part.is_none());
    }

    #[test]
    fn test_fraction_part_count_zeroes() {
        assert_eq!(3, fraction_part_count_zeroes("123000"));
        assert_eq!(0, fraction_part_count_zeroes("123"));
        assert_eq!(1, fraction_part_count_zeroes("0"));
    }

    #[test]
    fn test_fraction_part_remove_zeroes() {
        assert_eq!("123", fraction_part_remove_zeroes("123000"));
        assert_eq!("123", fraction_part_remove_zeroes("123"));
        assert_eq!("", fraction_part_remove_zeroes("0"));
    }

    #[test]
    fn test_fraction_part_or_none() {
        assert_eq!(
            Some("123".to_owned()),
            normalized_fraction_part_or_none(Some("123000".to_owned()))
        );
        assert_eq!(
            Some("123".to_owned()),
            normalized_fraction_part_or_none(Some("123".to_owned()))
        );
        assert_eq!(None, normalized_fraction_part_or_none(Some("0".to_owned())));
        assert_eq!(
            None,
            normalized_fraction_part_or_none(Some("00000".to_owned()))
        );
        assert_eq!(None, normalized_fraction_part_or_none(None));
    }

    #[test]
    fn test_fmt_align_fraction_strings() {
        let res = fmt_align_fraction_strings(
            &vec!["-42", "0.3214", "1000", "-1000.2"].into_boxed_slice(),
        );
        assert_eq!("  -42", res[0]);
        assert_eq!("    0.3214", res[1]);
        assert_eq!(" 1000", res[2]);
        assert_eq!("-1000.2", res[3]);

        let res = fmt_align_fractions(
            &vec![
                FractionNumber::F64(-42.0),
                FractionNumber::F64(0.3214),
                FractionNumber::F64(1000.0),
                FractionNumber::F64(-1000.2),
            ]
            .into_boxed_slice(),
            4,
        );
        assert_eq!("  -42", res[0]);
        assert_eq!("    0.3214", res[1]);
        assert_eq!(" 1000", res[2]);
        assert_eq!("-1000.2", res[3]);
    }

    #[test]
    fn test_fmt_unnecessary_zeroes_are_removed() {
        let res = fmt_align_fraction_strings(&vec!["1.000000000"].into_boxed_slice());
        assert_eq!("1", res[0]);

        let res = fmt_align_fractions(
            &vec![FractionNumber::F32(1.0), FractionNumber::F64(1.0)].into_boxed_slice(),
            4,
        );
        assert_eq!("1", res[0]);
        assert_eq!("1", res[1]);
    }
}
