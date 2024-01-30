
#![allow(non_snake_case)]


#[test]
fn Test_Add1() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn Test_Add2() {
    let result = 4 + 7;
    assert_eq!(result, 11, "Testing addition of {} + {}", 4, 7);
}