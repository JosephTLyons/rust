// error-pattern:thread 'main' panicked at 'attempt to shift right with overflow'
// compile-flags: -C debug-assertions

#![warn(arithmetic_overflow)]
#![warn(const_err)]

fn main() {
    let _n = 1i64 >> [64][0];
}
