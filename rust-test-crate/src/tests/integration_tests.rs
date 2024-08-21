use crate::add_divide_multiply;

#[test]
fn test_add_divide_multiply() {
    let result = add_divide_multiply(6,3);
    assert_eq!(result, 29);
}