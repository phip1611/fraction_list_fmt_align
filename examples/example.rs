use fraction_list_fmt_align::{fmt_align_fraction_strings, FractionNumber};

fn main() {
    let input_1 = vec![
        "-42",
        "0.3214",
        "1000",
        "-1000.2",
    ];
    let aligned_1 = fmt_align_fraction_strings(&input_1);

    // or

    let input_1 = vec![
        FractionNumber::F32(123.456),
        FractionNumber::F64(123.456),
        FractionNumber::F64(0.0),
        FractionNumber::F64(-42.0),
        FractionNumber::F64(-42.0),
    ];
    let aligned_1 = fmt_align_fraction_strings(&input_1);
}