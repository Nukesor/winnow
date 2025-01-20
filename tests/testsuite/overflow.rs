#![allow(clippy::unreadable_literal)]
#![cfg(target_pointer_width = "64")]

#[cfg(feature = "alloc")]
use winnow::binary::be_u64;
#[cfg(feature = "alloc")]
use winnow::binary::length_take;
#[cfg(feature = "alloc")]
use winnow::combinator::repeat;
use winnow::error::{ErrMode, Needed};
use winnow::prelude::*;
use winnow::token::take;
use winnow::Partial;

// Parser definition

#[test]
fn overflow_incomplete_tuple() {
    // We request a length that would trigger an overflow if computing consumed + requested
    #[allow(clippy::type_complexity)]
    fn parser02<'i>(i: &mut Partial<&'i [u8]>) -> PResult<(&'i [u8], &'i [u8])> {
        (take(1_usize), take(18446744073709551615_usize)).parse_next(i)
    }

    assert_eq!(
        parser02.parse_peek(Partial::new(&b"3"[..])),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551615)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_length_bytes() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., length_take(be_u64)).parse_next(i)
    }

    // Trigger an overflow in length_take
    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xff"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551615)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_many0() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., length_take(be_u64)).parse_next(i)
    }

    // Trigger an overflow in repeat
    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xef"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551599)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_many1() {
    use winnow::combinator::repeat;

    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(1.., length_take(be_u64)).parse_next(i)
    }

    // Trigger an overflow in repeat
    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xef"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551599)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_many_till0() {
    use winnow::combinator::repeat_till;

    #[allow(clippy::type_complexity)]
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<(Vec<&'i [u8]>, &'i [u8])> {
        repeat_till(0.., length_take(be_u64), "abc").parse_next(i)
    }

    // Trigger an overflow in repeat_till
    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xef"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551599)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_many_m_n() {
    use winnow::combinator::repeat;

    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(2..=4, length_take(be_u64)).parse_next(i)
    }

    // Trigger an overflow in repeat
    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xef"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551599)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_count() {
    fn counter<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(2, length_take(be_u64)).parse_next(i)
    }

    assert_eq!(
        counter.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xef"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551599)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_length_repeat() {
    use winnow::binary::be_u8;
    use winnow::binary::length_repeat;

    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        length_repeat(be_u8, length_take(be_u64)).parse_next(i)
    }

    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x04\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xee"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551598)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn overflow_incomplete_length_take() {
    fn multi<'i>(i: &mut Partial<&'i [u8]>) -> PResult<Vec<&'i [u8]>> {
        repeat(0.., length_take(be_u64)).parse_next(i)
    }

    assert_eq!(
        multi.parse_peek(Partial::new(
            &b"\x00\x00\x00\x00\x00\x00\x00\x01\xaa\xff\xff\xff\xff\xff\xff\xff\xff"[..]
        )),
        Err(ErrMode::Incomplete(Needed::new(18446744073709551615)))
    );
}
