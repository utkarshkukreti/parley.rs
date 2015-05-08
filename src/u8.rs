use Parser;

pub struct Satisfy<F: FnMut(u8) -> bool> {
    f: F
}

pub fn satisfy<F: FnMut(u8) -> bool>(f: F) -> Satisfy<F> {
    Satisfy {
        f: f
    }
}

impl<F> Parser for Satisfy<F>
    where F: FnMut(u8) -> bool
{
    type Output = u8;
    fn parse(&mut self, input: &[u8]) -> Result<(u8, usize), ()> {
        if let Some(&b) = input.get(0) {
            if (self.f)(b) {
                return Ok((b, 1));
            }
        }
        Err(())
    }
}

pub struct U8 {
    u8: u8
}

impl Parser for U8 {
    type Output = u8;
    fn parse(&mut self, input: &[u8]) -> Result<(u8, usize), ()> {
        if input.get(0) == Some(&self.u8) {
            return Ok((self.u8, 1));
        }
        Err(())
    }
}

#[test]
fn test_satisfy() {
    let mut x = satisfy(|b| b == b'x');
    assert_eq!(x.parse(b""), Err(()));
    assert_eq!(x.parse(b"x"), Ok((b'x', 1)));
    assert_eq!(x.parse(b"y"), Err(()));
    assert_eq!(x.parse(b"xy"), Ok((b'x', 1)));
}

#[test]
fn test_u8() {
    let mut x = U8 {
        u8: b'x'
    };
    assert_eq!(x.parse(b""), Err(()));
    assert_eq!(x.parse(b"x"), Ok((b'x', 1)));
    assert_eq!(x.parse(b"y"), Err(()));
    assert_eq!(x.parse(b"xy"), Ok((b'x', 1)));
}
