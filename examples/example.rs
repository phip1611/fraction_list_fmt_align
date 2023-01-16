use fraction_list_fmt_align::{
    fmt_align_fraction_strings, fmt_align_fractions, FormatPrecision, FractionNumber,
};

fn main() {
    let input_1 = vec!["-42", "0.3214", "1000", "-1000.2", "2.00000"];
    let aligned_1 = fmt_align_fraction_strings(&input_1);
    println!("{:#?}", aligned_1);

    // or

    let input_2 = vec![
        FractionNumber::F32(-42.0),
        FractionNumber::F64(0.3214),
        FractionNumber::F64(1000.0),
        FractionNumber::F64(-1000.2),
        FractionNumber::F64(2.00000),
    ];
    let max_precision = 4;
    let aligned_2 = fmt_align_fractions(&input_2, FormatPrecision::Max(max_precision));
    println!("{:#?}", aligned_2);
}
