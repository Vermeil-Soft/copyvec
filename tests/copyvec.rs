#[cfg(feature = "serde")]
use serde_test::{assert_tokens, assert_de_tokens_error, Token};

use copyvec::*;

#[test]
fn test_macro() {
    let mut expected: CopyVec<i32, 4> = CopyVec::new();
    expected.push(1);
    expected.push(2);
    expected.push(3);

    assert_eq!(copy_vec!(4 => 1, 2, 3), expected);
}

#[test]
fn test_push_pop() {
    let mut subject: CopyVec<i32, 4> = CopyVec::new();
    assert_eq!(copy_vec!(4), subject);
    subject.push(1);
    assert_eq!(copy_vec!(4 => 1), subject);
    subject.push(2);
    assert_eq!(copy_vec!(4 => 1, 2), subject);
    subject.push(3);
    assert_eq!(copy_vec!(4 => 1, 2, 3), subject);
    subject.push(4);
    assert_eq!(copy_vec!(4 => 1, 2, 3, 4), subject);
    assert_eq!(subject.pop(), Some(4));
    assert_eq!(subject.pop(), Some(3));
    assert_eq!(subject.pop(), Some(2));
    assert_eq!(subject.pop(), Some(1));
    assert_eq!(subject.pop(), None);
    assert_eq!(copy_vec!(4), subject);
}

#[test]
fn test_slice() {
    let mut testvec: CopyVec<i32, 4> = copy_vec!(4 => 1, 2, 3, 4);

    {
        let slice = testvec.as_slice();
        assert_eq!(slice[3], 4);
        assert_eq!(slice[2], 3);
        assert_eq!(slice[1], 2);
        assert_eq!(slice[0], 1);
    }
    {
        let slice_mut = testvec.as_mut_slice();
        assert_eq!(slice_mut[3], 4);
        assert_eq!(slice_mut[2], 3);
        assert_eq!(slice_mut[1], 2);
        assert_eq!(slice_mut[0], 1);
    }
}

#[test]
fn test_clear() {
    let mut testvec: CopyVec<i32, 4> = copy_vec!(4 => 1, 2, 3, 4);
    testvec.clear();
    assert_eq!(testvec.pop(), None);
    testvec.push(5);
    testvec.push(6);
    testvec.push(7);
    testvec.push(8);
    assert_eq!(copy_vec!(4 => 5, 6, 7, 8), testvec);
}

#[test]
fn test_pop_at() {
    let mut testvec: CopyVec<i32, 4> = copy_vec!(4 => 1, 2, 3, 4);
    assert_eq!(testvec.pop_at(5), None);
    assert_eq!(testvec.pop_at(0), Some(1));
    assert_eq!(testvec.pop_at(2), Some(4));
    assert_eq!(testvec.pop_at(2), None);
    assert_eq!(testvec.pop_at(1), Some(3));
    assert_eq!(testvec.pop_at(0), Some(2));
}

#[test]
#[should_panic]
fn test_macro_overflow() {
    let _v = copy_vec!(4 => 1, 2, 3, 4, 5);
}

#[test]
#[should_panic]
fn test_push_overflow() {
    let mut subject: CopyVec<i32, 4> = CopyVec::new();
    subject.push(1);
    subject.push(2);
    subject.push(3);
    subject.push(4);
    subject.push(5);
}

#[test]
#[cfg(feature = "serde")]
fn test_ser_de() {
    let v0: CopyVec<i32, 4> = copy_vec!(4);
    let v1 = copy_vec!(4 => 1);
    let v2 = copy_vec!(4 => 1, 2);
    let v3 = copy_vec!(4 => 1, 2, 3);
    let v4 = copy_vec!(4 => 1, 2, 3, 4);

    assert_tokens(&v0, &[
        Token::Seq { len: Some(0) },
        Token::SeqEnd,
    ]);
    assert_tokens(&v1, &[
        Token::Seq { len: Some(1) },
        Token::I32(1),
        Token::SeqEnd,
    ]);
    assert_tokens(&v2, &[
        Token::Seq { len: Some(2) },
        Token::I32(1),
        Token::I32(2),
        Token::SeqEnd,
    ]);
    assert_tokens(&v3, &[
        Token::Seq { len: Some(3) },
        Token::I32(1),
        Token::I32(2),
        Token::I32(3),
        Token::SeqEnd,
    ]);
    assert_tokens(&v4, &[
        Token::Seq { len: Some(4) },
        Token::I32(1),
        Token::I32(2),
        Token::I32(3),
        Token::I32(4),
        Token::SeqEnd,
    ]);
}

#[test]
#[cfg(feature = "serde")]
fn test_deser_error() {
    assert_de_tokens_error::<CopyVec<i32, 4>>(&[
            Token::Seq { len: Some(5) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::I32(5),
            Token::SeqEnd,
        ],
        "invalid length 5, expected a sequence with at most 4 elements"
    );
}