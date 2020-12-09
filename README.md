## Rust library: fraction_numbers_fmt_align
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

# Output
"  -42"
"    0.3214"
" 1000"
"-1000.2"
```

## Use case
If you want to write multiple fraction numbers of different
lengths to the terminal or a file.

## How to use
#### Cargo.toml
```

```

## Trivia
I can't believe it takes so much to solve such a simple problem.
Did I oversee something? If you find a simpler solution 
please message me :)
