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
//! "2.00000"
//! ```
//! ## Example Output
//! ```text
//! "  -42     "
//! "    0.3214"
//! " 1000     "
//! "-1000.2   "
//! "    2     "

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::fallible_impl_from,
    clippy::needless_doctest_main,
    clippy::redundant_pub_crate,
    clippy::suboptimal_flops
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

/// Abstraction over floating point types [`f32`] and [`f64`].
#[derive(Debug, Copy, Clone)]
pub enum FractionNumber {
    /// Variant for [`f32`].
    F32(f32),
    /// Variant for [`f64`].
    F64(f64),
}

impl From<f32> for FractionNumber {
    fn from(val: f32) -> Self {
        Self::F32(val)
    }
}

impl From<f64> for FractionNumber {
    fn from(val: f64) -> Self {
        Self::F64(val)
    }
}

impl FractionNumber {
    fn format(self, precision: FormatPrecision) -> String {
        match self {
            Self::F32(val) => {
                format!("{val:.precision$}", val = val, precision = precision.val())
            }
            Self::F64(val) => {
                format!("{val:.precision$}", val = val, precision = precision.val())
            }
        }
    }
}

/// The precision of decimal places for [`fmt_align_fractions`].
#[derive(Copy, Clone, Debug)]
pub enum FormatPrecision {
    /// Format with exactly `n` decimal places.
    Exact(u8),
    /// Format with a maximum of `n` decimal places. Might happen that there is not a
    /// single decimal place required.
    Max(u8),
}

impl FormatPrecision {
    const fn val(self) -> usize {
        let val = match self {
            Self::Exact(val) => val,
            Self::Max(val) => val,
        };
        val as usize
    }
}

/// Convenient wrapper around [`fmt_align_fraction_strings`] that takes
/// a slice of floating point values, formats them all with a maximum
/// precision and returns a list of aligned, formatted strings.
/// * `precision`: See [`FormatPrecision`].
pub fn fmt_align_fractions(
    fractions: &[FractionNumber],
    precision: FormatPrecision,
) -> Vec<String> {
    let fraction_strings = fractions
        .iter()
        .map(|fr| fr.format(precision))
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
///
/// This takes the precision from the longest fractional part (without unnecessary zeroes)
/// to align all strings.
///
/// ## Example Input
/// ```text
/// "-42"
/// "0.3214"
/// "1000"
/// "-1000.2"
/// "2.00000"
/// ```
/// ## Example Output
/// ```text
/// "  -42     "
/// "    0.3214"
/// " 1000     "
/// "-1000.2   "
/// "    2     "
/// ```
pub fn fmt_align_fraction_strings(strings: &[&str]) -> Vec<String> {
    // normalize all fractional parts
    let strings = strings
        .iter()
        .map(|x| normalize_fraction_part(x))
        .collect::<Vec<&str>>();

    let max = strings
        .iter()
        .map(|x| get_whole_part(x))
        .map(|x| x.len())
        .max()
        .unwrap();

    // create n new strings
    let mut new_strings = vec![String::new(); strings.len()];
    strings.iter().enumerate().for_each(|(index, string)| {
        let whole_part = get_whole_part(string);
        let spaces = max - whole_part.len();
        new_strings[index].push_str(&" ".repeat(spaces));
        new_strings[index].push_str(string);
    });

    // now add spaces in the end so that all are exactly same aligned, on left
    // as well as right; technically this is not really needed, but it may
    // help in some situations. Also this can be easily revoked with a right trim.
    let max = new_strings.iter().map(|s| s.len()).max().unwrap();
    for string in &mut new_strings {
        let spaces = max - string.len();
        string.push_str(&" ".repeat(spaces))
    }

    new_strings
}

/// Get the whole part (TODO is this the right term?)
/// from a formatted fraction number string.
/// * `123` => `123`
/// * `123.13` => `123`
/// * `0.1234` => `0`
/// * `-10.1234` => `-10`
fn get_whole_part(string: &str) -> &str {
    // if it doesn't contain "." the whole thing is returned
    string.split('.').next().unwrap()
}

/// Get the fractional part from a formatted fraction number string.
/// * `123` => `None`
/// * `123.13` => `Some(13)`
/// * `0.1234` => `Some(1234)`
/// * `-10.1234` => `Some(1234)`
fn get_fractional_part(string: &str) -> Option<&str> {
    let mut split = string.split('.');
    let _whole_part = split.next().unwrap();
    split.next()
}

/// Consumes the whole number string and normalizes
/// (if present) the fraction part. This means:
/// * `123` => `123`
/// * `123.13` => `123.13`
/// * `0.1234000` => `0.1234`
/// * `-10.000000` => `-10`
fn normalize_fraction_part(string: &str) -> &str {
    let whole_part = get_whole_part(string);
    let fraction_part = get_fractional_part(string);
    if fraction_part.is_none() {
        return whole_part;
    }
    let fraction_part = fraction_part.unwrap();
    let zeroes = fraction_part_count_zeroes(fraction_part);
    if fraction_part.len() == zeroes {
        whole_part
    } else {
        &string[0..string.len() - zeroes]
    }
}

/// Takes only the fraction part of a string without ".".
/// Counts that in "123000" (fractional part of "0.123000") are three unnecessary zeroes.
/// In "0.0000" there are four unnecessary zeroes.
fn fraction_part_count_zeroes(fraction_part: &str) -> usize {
    let mut zeroes = 0;
    let chars = fraction_part.chars().collect::<Vec<char>>();
    for i in 0..fraction_part.len() {
        // go backwards
        let i = fraction_part.len() - 1 - i;
        let char = chars[i];
        if char == '0' {
            zeroes += 1;
        } else {
            break;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fraction_part_count_zeroes() {
        assert_eq!(3, fraction_part_count_zeroes("123000"));
        assert_eq!(0, fraction_part_count_zeroes("123"));
        assert_eq!(1, fraction_part_count_zeroes("0"));
        assert_eq!(11, fraction_part_count_zeroes("00000012800000000000"));
    }

    #[test]
    fn test_fmt_align_fraction_strings() {
        let res = fmt_align_fraction_strings(
            &vec!["-42", "0.3214", "1000", "-1000.2"].into_boxed_slice(),
        );
        assert_eq!("  -42     ", res[0]);
        assert_eq!("    0.3214", res[1]);
        assert_eq!(" 1000     ", res[2]);
        assert_eq!("-1000.2   ", res[3]);

        let res = fmt_align_fractions(
            &vec![
                FractionNumber::F64(-42.0),
                FractionNumber::F64(0.3214),
                FractionNumber::F64(1000.0),
                FractionNumber::F64(-1000.2),
            ]
            .into_boxed_slice(),
            FormatPrecision::Max(4),
        );
        assert_eq!("  -42     ", res[0]);
        assert_eq!("    0.3214", res[1]);
        assert_eq!(" 1000     ", res[2]);
        assert_eq!("-1000.2   ", res[3]);
    }

    #[test]
    fn test_fmt_unnecessary_zeroes_are_removed() {
        let res = fmt_align_fraction_strings(&vec!["1.000000000"].into_boxed_slice());
        assert_eq!("1", res[0]);

        let res = fmt_align_fractions(
            &vec![FractionNumber::F32(1.0), FractionNumber::F64(1.0)].into_boxed_slice(),
            FormatPrecision::Max(4),
        );
        assert_eq!("1", res[0]);
        assert_eq!("1", res[1]);
    }

    // tests that we get "NaN" and not a panic or so
    #[test]
    fn test_fmt_nan() {
        let res = fmt_align_fractions(
            &[FractionNumber::F32(f32::NAN), FractionNumber::F64(f64::NAN)],
            FormatPrecision::Max(20), // not important here
        );

        assert_eq!("NaN", res[0]);
        assert_eq!("NaN", res[1]);
    }
}
