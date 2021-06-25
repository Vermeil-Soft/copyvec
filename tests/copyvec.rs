#![allow(bad_style)]

use copyvec::*;

#[test]
fn test_macro() {
    let mut expected: CopyVec<i32, 4> = CopyVec::new();
    expected.push(1);
    expected.push(2);
    expected.push(3);

    assert_eq!(copy_vec!(4 => 1, 2, 3), expected);
}