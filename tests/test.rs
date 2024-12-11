use inttype_enum::IntType;


#[derive(Debug, PartialEq, Eq)]
#[derive(IntType)]
#[repr(u8)]
enum Test {
    Hello = 255,
}

#[test]
fn test() {
    assert_eq!(Test::try_from(255), Ok(Test::Hello));
    assert_eq!(Test::try_from(0), Err(0));
}