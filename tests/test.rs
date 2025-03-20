use inttype_enum::*;


#[derive(Debug, PartialEq, Eq)]
#[derive(IntType)]
#[repr(u8)]
enum Test {
    Hello = 255,
}

#[derive(NewIntType)]
#[repr(u8)]
enum Test2 {
    A = 0x00,
    #[range(10..16)]
    B(u8),
    C = 2,
}

#[test]
fn test() {
    assert_eq!(Test::try_from(255), Ok(Test::Hello));
    assert_eq!(Test::try_from(0), Err(0));
}