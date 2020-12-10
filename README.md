## Rust library: fraction_list_fmt_align
Formats a list of arbitrary fractional numbers (either string 
or f32/f64) so that they are correctly aligned when printed 
line by line. It also removes unnecessary zeroes. This means that
it rather returns "7" instead of "7.000000".

## Input
a) either **a list of formatted fractional number strings**

b) or **a list of f32/f64**

## Example
``` 
# Input
"-42"
"0.3214"
"1000"
"-1000.2"
"2.00000"

# Output
"  -42     "
"    0.3214"
" 1000     "
"-1000.2   "
"    2     "
```

## Use case
If you want to write multiple fraction numbers of different
lengths to the terminal or a file in an aligned/formatted way.

## How to use
```rust
use fraction_list_fmt_align::{fmt_align_fraction_strings, FractionNumber, fmt_align_fractions};

fn main() {
    let input_1 = vec![
        "-42",
        "0.3214",
        "1000",
        "-1000.2",
    ];
    let aligned_1 = fmt_align_fraction_strings(&input_1);
    println!("{:#?}", aligned_1);

    // or
    let input_2 = vec![
        FractionNumber::F32(-42.0),
        FractionNumber::F64(0.3214),
        FractionNumber::F64(1000.0),
        FractionNumber::F64(-1000.2),
    ];
    let max_precision = 4;
    let aligned_2 = fmt_align_fractions(&input_2, max_precision);
    println!("{:#?}", aligned_2);
}
```

## Difference to `std::fmt`
* This is more flexible and adjusts to a dynamic precission
* his removes unnecessary zeroes, i.e. "0.000" will become "0"
