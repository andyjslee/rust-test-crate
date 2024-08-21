use crate::multiply;

#[test]
fn test_multiply() {
    let result = multiply(10,2);
    assert_eq!(result, 20);
}