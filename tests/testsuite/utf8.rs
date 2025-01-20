use winnow::ascii::Caseless;
use winnow::prelude::*;
use winnow::Partial;
#[cfg(feature = "alloc")]
use winnow::{combinator::alt, combinator::repeat, token::literal};
use winnow::{
    error::ErrMode,
    error::{ErrorKind, IResult, InputError},
    token::{take, take_till, take_until, take_while},
};

#[test]
fn literal_succeed_str() {
    const INPUT: &str = "Hello World!";
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        "Hello".parse_next(input)
    }

    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == " World!",
                "Parser `literal` consumed leftover input."
            );
            assert!(
                output == "Hello",
                "Parser `literal` doesn't return the literal it matched on success. \
           Expected `{}`, got `{}`.",
                "Hello",
                output
            );
        }
        other => panic!(
            "Parser `literal` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn literal_incomplete_str() {
    const INPUT: &str = "Hello";

    let res: IResult<_, _, InputError<_>> = "Hello World!".parse_peek(Partial::new(INPUT));
    match res {
        Err(ErrMode::Incomplete(_)) => (),
        other => {
            panic!(
                "Parser `literal` didn't require more input when it should have. \
           Got `{other:?}`."
            );
        }
    };
}

#[test]
fn literal_error_str() {
    const INPUT: &str = "Hello World!";

    let res: IResult<_, _, InputError<_>> = "Random".parse_peek(INPUT);
    match res {
        Err(ErrMode::Backtrack(_)) => (),
        other => {
            panic!("Parser `literal` didn't fail when it should have. Got `{other:?}`.`");
        }
    };
}

#[cfg(feature = "alloc")]
#[test]
fn literal_case_insensitive_str() {
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        literal(Caseless("ABcd")).parse_next(input)
    }
    assert_eq!(test.parse_peek("aBCdefgh"), Ok(("efgh", "aBCd")));
    assert_eq!(test.parse_peek("abcdefgh"), Ok(("efgh", "abcd")));
    assert_eq!(test.parse_peek("ABCDefgh"), Ok(("efgh", "ABCD")));
}

#[test]
fn take_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";

    let res: IResult<_, _, InputError<_>> = take(9_usize).parse_peek(INPUT);
    match res {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_s` consumed leftover input. Leftover `{extra}`."
            );
            assert!(
          output == CONSUMED,
          "Parser `take_s` doesn't return the string it consumed on success. Expected `{CONSUMED}`, got `{output}`."
        );
        }
        other => panic!(
            "Parser `take_s` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_incomplete_str() {
    use winnow::token::take;

    const INPUT: &str = "βèƒôřèÂßÇá";

    let res: IResult<_, _, InputError<_>> = take(13_usize).parse_peek(Partial::new(INPUT));
    match res {
        Err(ErrMode::Incomplete(_)) => (),
        other => panic!(
            "Parser `take` didn't require more input when it should have. \
         Got `{other:?}`."
        ),
    }
}

#[test]
fn take_until_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇ∂áƒƭèř";
    const FIND: &str = "ÂßÇ∂";
    const CONSUMED: &str = "βèƒôřè";
    const LEFTOVER: &str = "ÂßÇ∂áƒƭèř";

    let res: IResult<_, _, InputError<_>> = take_until(0.., FIND).parse_peek(INPUT);
    match res {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_until`\
           consumed leftover input. Leftover `{extra}`."
            );
            assert!(
                    output == CONSUMED,
                    "Parser `take_until`\
           doesn't return the string it consumed on success. Expected `{CONSUMED}`, got `{output}`."
                );
        }
        other => panic!(
            "Parser `take_until` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_until_incomplete_str() {
    use winnow::token::take_until;

    const INPUT: &str = "βèƒôřè";
    const FIND: &str = "βèƒôřèÂßÇ";

    let res: IResult<_, _, InputError<_>> = take_until(0.., FIND).parse_peek(Partial::new(INPUT));
    match res {
        Err(ErrMode::Incomplete(_)) => (),
        other => panic!(
            "Parser `take_until` didn't require more input when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_until_error_str() {
    use winnow::token::take_until;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const FIND: &str = "Ráñδô₥";

    let res: IResult<_, _, InputError<_>> = take_until(0.., FIND).parse_peek(Partial::new(INPUT));
    match res {
        Err(ErrMode::Incomplete(_)) => (),
        other => panic!(
            "Parser `take_until` didn't fail when it should have. \
         Got `{other:?}`."
        ),
    };
}

fn is_alphabetic(c: char) -> bool {
    (c as u8 >= 0x41 && c as u8 <= 0x5A) || (c as u8 >= 0x61 && c as u8 <= 0x7A)
}

#[test]
fn take_while_str() {
    use winnow::error::Needed;

    use winnow::token::take_while;

    fn f<'i>(input: &mut Partial<&'i str>) -> PResult<&'i str, InputError<Partial<&'i str>>> {
        take_while(0.., is_alphabetic).parse_next(input)
    }
    let a = "";
    let b = "abcd";
    let c = "abcd123";
    let d = "123";

    assert_eq!(
        f.parse_peek(Partial::new(a)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        f.parse_peek(Partial::new(b)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(f.parse_peek(Partial::new(c)), Ok((Partial::new(d), b)));
    assert_eq!(f.parse_peek(Partial::new(d)), Ok((Partial::new(d), a)));
}

#[test]
fn take_while_succeed_none_str() {
    use winnow::token::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "";
    const LEFTOVER: &str = "βèƒôřèÂßÇáƒƭèř";
    fn while_s(c: char) -> bool {
        c == '9'
    }
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(0.., while_s).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_while` consumed leftover input."
            );
            assert!(
                output == CONSUMED,
                "Parser `take_while` doesn't return the string it consumed on success. \
           Expected `{CONSUMED}`, got `{output}`."
            );
        }
        other => panic!(
            "Parser `take_while` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_while_succeed_some_str() {
    use winnow::token::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn while_s(c: char) -> bool {
        matches!(c, 'β' | 'è' | 'ƒ' | 'ô' | 'ř' | 'Â' | 'ß' | 'Ç')
    }
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(0.., while_s).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_while` consumed leftover input."
            );
            assert!(
                output == CONSUMED,
                "Parser `take_while` doesn't return the string it consumed on success. \
           Expected `{CONSUMED}`, got `{output}`."
            );
        }
        other => panic!(
            "Parser `take_while` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn test_take_while1_str() {
    use winnow::error::Needed;

    fn f<'i>(input: &mut Partial<&'i str>) -> PResult<&'i str, InputError<Partial<&'i str>>> {
        take_while(1.., is_alphabetic).parse_next(input)
    }
    let a = "";
    let b = "abcd";
    let c = "abcd123";
    let d = "123";

    assert_eq!(
        f.parse_peek(Partial::new(a)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(
        f.parse_peek(Partial::new(b)),
        Err(ErrMode::Incomplete(Needed::new(1)))
    );
    assert_eq!(f.parse_peek(Partial::new(c)), Ok((Partial::new("123"), b)));
    assert_eq!(
        f.parse_peek(Partial::new(d)),
        Err(ErrMode::Backtrack(InputError::new(
            Partial::new(d),
            ErrorKind::Slice
        )))
    );
}

#[test]
fn take_while1_fn_succeed_str() {
    use winnow::token::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn while1_s(c: char) -> bool {
        matches!(c, 'β' | 'è' | 'ƒ' | 'ô' | 'ř' | 'Â' | 'ß' | 'Ç')
    }
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(1.., while1_s).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_while` consumed leftover input."
            );
            assert!(
                output == CONSUMED,
                "Parser `take_while` doesn't return the string it consumed on success. \
           Expected `{CONSUMED}`, got `{output}`."
            );
        }
        other => panic!(
            "Parser `take_while` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_while1_set_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const MATCH: &[char] = &['β', 'è', 'ƒ', 'ô', 'ř', 'è', 'Â', 'ß', 'Ç'];
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(1.., MATCH).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `is_a` consumed leftover input. Leftover `{extra}`."
            );
            assert!(
          output == CONSUMED,
          "Parser `is_a` doesn't return the string it consumed on success. Expected `{CONSUMED}`, got `{output}`."
        );
        }
        other => panic!(
            "Parser `is_a` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_while1_fn_fail_str() {
    use winnow::token::take_while;

    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    fn while1_s(c: char) -> bool {
        c == '9'
    }
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(1.., while1_s).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Err(ErrMode::Backtrack(_)) => (),
        other => panic!(
            "Parser `take_while` didn't fail when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_while1_set_fail_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const MATCH: &[char] = &['Û', 'ñ', 'ℓ', 'ú', 'ç', 'ƙ', '¥'];
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_while(1.., MATCH).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Err(ErrMode::Backtrack(_)) => (),
        other => panic!("Parser `is_a` didn't fail when it should have. Got `{other:?}`."),
    };
}

#[test]
fn take_till0_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn till_s(c: char) -> bool {
        c == 'á'
    }
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_till(0.., till_s).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_till0` consumed leftover input."
            );
            assert!(
                output == CONSUMED,
                "Parser `take_till0` doesn't return the string it consumed on success. \
           Expected `{CONSUMED}`, got `{output}`."
            );
        }
        other => panic!(
            "Parser `take_till0` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_till1_succeed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const AVOID: &[char] = &['£', 'ú', 'ç', 'ƙ', '¥', 'á'];
    const CONSUMED: &str = "βèƒôřèÂßÇ";
    const LEFTOVER: &str = "áƒƭèř";
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_till(1.., AVOID).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Ok((extra, output)) => {
            assert!(
                extra == LEFTOVER,
                "Parser `take_till1` consumed leftover input. Leftover `{extra}`."
            );
            assert!(
          output == CONSUMED,
          "Parser `take_till1` doesn't return the string it consumed on success. Expected `{CONSUMED}`, got `{output}`."
        );
        }
        other => panic!(
            "Parser `take_till1` didn't succeed when it should have. \
         Got `{other:?}`."
        ),
    };
}

#[test]
fn take_till1_failed_str() {
    const INPUT: &str = "βèƒôřèÂßÇáƒƭèř";
    const AVOID: &[char] = &['β', 'ú', 'ç', 'ƙ', '¥'];
    fn test<'i>(input: &mut &'i str) -> PResult<&'i str> {
        take_till(1.., AVOID).parse_next(input)
    }
    match test.parse_peek(INPUT) {
        Err(ErrMode::Backtrack(_)) => (),
        other => panic!("Parser `is_not` didn't fail when it should have. Got `{other:?}`."),
    };
}

#[test]
#[cfg(feature = "alloc")]
fn take_is_a_str() {
    use winnow::prelude::*;

    let a = "aabbab";
    let b = "ababcd";

    fn f<'i>(input: &mut &'i str) -> PResult<&'i str> {
        repeat::<_, _, (), _, _>(1.., alt(("a", "b")))
            .take()
            .parse_next(input)
    }

    assert_eq!(f.parse_peek(a), Ok((&a[6..], a)));
    assert_eq!(f.parse_peek(b), Ok((&b[4..], &b[..4])));
}

#[test]
fn utf8_indexing_str() {
    fn dot<'i>(input: &mut &'i str) -> PResult<&'i str> {
        ".".parse_next(input)
    }

    let _ = dot.parse_peek("點");
}
