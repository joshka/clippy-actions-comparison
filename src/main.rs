fn main() {
    // an intentional clippy failure (clippy::absurd_extreme_comparisons)
    // <https://rust-lang.github.io/rust-clippy/stable/index.html#/absurd_extreme_comparisons>
    if 0 > i32::MAX {
        println!("0 is greater than i32::MAX");
    }

    // clippy::assertions_on_constants
    // <https://rust-lang.github.io/rust-clippy/stable/index.html#/assertions_on_constants>
    assert!(false);
}
