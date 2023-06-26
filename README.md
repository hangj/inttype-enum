# inttype-enum

Converts an [`enum`] into an [`inttype`], and try to convert it back

Usage example:  
```rs
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
