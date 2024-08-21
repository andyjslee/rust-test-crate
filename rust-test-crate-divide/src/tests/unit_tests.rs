use crate::divide;

#[test]
fn test_divide() {
    let result = divide(10,2);
    assert_eq!(result, 5);
}