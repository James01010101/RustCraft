
#![allow(non_snake_case)]

extern crate GameEngine;

use GameEngine::TestFuncs::{Add};


#[test]
fn Test_Add1() {
    let result = Add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn Test_Add2() {
    let result = Add(4, 7);
    assert_eq!(result, 11);
}

#[test]
fn Test_Add3() {
    let result = Add(2, 0);
    assert_eq!(result, 2);
}