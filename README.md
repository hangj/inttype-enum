# inttype-enum
Converts an [`enum`] into an [`inttype`], and try to convert it back  


Auto implement `From<enum> for inttype`, and `TryFrom<inttype> for enum`.
if one(only one) variant is tagged with `#[default]`, then `From<inttype> for enum` will be implemented


Usage examples:  

```toml
[dependencies]
inttype-enum = "0.2"
```

```rust
use inttype_enum::IntType;

#[derive(IntType)]
#[repr(u8)]
enum Cmd {
    Connect = 1,
    Bind = 2,
    Udp = 3,
}

let conn: u8 = Cmd::Connect.into();
assert!(matches!(Cmd::try_from(conn), Ok(Cmd::Connect)));
assert!(matches!(Cmd::try_from(0), Err(_)));
```

```rust
use inttype_enum::IntType;

#[derive(IntType)]
#[repr(u8)]
enum Method {
    A = 1,
    B = 2,
    #[default]
    C = 3,
}
assert!(matches!(1.into(), Method::A));
assert!(matches!(0.into(), Method::C));
```

```rust
use inttype_enum::IntRange;

#[repr(u8)]
#[derive(IntRange)]
#[derive(Debug, PartialEq, Eq)]
enum Test {
    A = 0x00,
    #[range(1..16)]
    B(u8),
    #[range(16..)]
    C(u8),
}

assert_eq!(Test::try_from(0), Ok(Test::A));
assert!(0u8 == Test::A.into());
assert_eq!(Test::B(16).is_valid(), false);
assert_eq!(Test::try_from(16), Ok(Test::C(16)));
```