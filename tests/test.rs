use inttype_enum::*;

#[derive(Debug, PartialEq, Eq, IntType)]
#[repr(u8)]
enum Test {
    Hello = 255,
}

#[derive(Debug, PartialEq, Eq, IntRange)]
#[repr(u8)]
enum Test2 {
    A = 0x00,
    #[range(1..16)]
    B(u8),
    #[range(16..)]
    C(u8),
}

#[derive(Debug, PartialEq, Eq, IntRange)]
#[repr(u8)]
enum Test3 {
    #[range(..)]
    A(u8),
}

#[derive(Debug, PartialEq, Eq, IntRange)]
#[repr(u8)]
enum Test4 {
    #[range(..128)]
    A(u8),
    #[range(128..255)]
    B(u8),
}

#[test]
fn test() {
    assert_eq!(Test::try_from(255), Ok(Test::Hello));
    assert_eq!(Test::try_from(0), Err(0));

    assert!(0u8 == Test2::A.into());
    assert!(0u8 == u8::from(Test2::A));
    assert_eq!(u8::from(Test2::B(11)), 11);
    assert_eq!(u8::from(Test2::B(16)), 16);
    assert_eq!(Test2::B(16).is_valid(), false);

    assert_eq!(Test2::from(16), Test2::C(16));
    assert_eq!(Test2::try_from(16), Ok(Test2::C(16)));
    assert!(255u8 == Test3::from(255).into());

    assert_eq!(Test4::try_from(0), Ok(Test4::A(0)));
    assert_eq!(Test4::try_from(16), Ok(Test4::A(16)));
    assert_eq!(Test4::try_from(127), Ok(Test4::A(127)));
    assert_eq!(Test4::try_from(128), Ok(Test4::B(128)));
    assert_eq!(Test4::try_from(255), Err(255));
}
