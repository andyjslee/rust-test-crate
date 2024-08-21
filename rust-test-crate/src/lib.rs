#[cfg(test)]
mod tests;

use rust_test_crate_add::add;
use rust_test_crate_divide::divide;
use rust_test_crate_multiply::multiply;

pub fn add_divide_multiply(a: isize, b: isize) -> isize {
    return add(a,b) + divide(a,b) + multiply(a,b);
}
