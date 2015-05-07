pub trait Parser: Sized {
    type Output;

    fn parse(&mut self, input: &[u8]) -> Result<(Self::Output, usize), ()>;

    fn then<P2: Parser>(self, other: P2) -> Then<Self, P2> {
        Then {
            p1: self,
            p2: other
        }
    }

    fn or<P2>(self, other: P2) -> Or<Self, P2>
        where P2: Parser<Output = Self::Output>
    {
        Or {
            p1: self,
            p2: other
        }
    }
}

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

pub struct Then<P1: Parser, P2: Parser> {
    p1: P1,
    p2: P2
}

impl<P1, P2> Parser for Then<P1, P2>
    where P1: Parser, P2: Parser
{
    type Output = (P1::Output, P2::Output);
    fn parse(&mut self, input: &[u8])
             -> Result<((P1::Output, P2::Output), usize), ()> {
        self.p1.parse(input).and_then(|(r1, c1)| {
            self.p2.parse(&input[c1..]).and_then(|(r2, c2)| {
                Ok(((r1, r2), c1 + c2))
            })
        })
    }
}

pub struct Or<P1: Parser, P2: Parser> {
    p1: P1,
    p2: P2
}

impl<O, P1, P2> Parser for Or<P1, P2>
    where P1: Parser<Output = O>, P2: Parser<Output = O>
{
    type Output = O;
    fn parse(&mut self, input: &[u8])
             -> Result<(O, usize), ()> {
        self.p1.parse(input).or_else(|_| self.p2.parse(input))
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
fn test_then() {
    let x = satisfy(|b| b == b'x');
    let y = satisfy(|b| b == b'y');
    let mut xy = x.then(y);
    assert_eq!(xy.parse(b""), Err(()));
    assert_eq!(xy.parse(b"x"), Err(()));
    assert_eq!(xy.parse(b"y"), Err(()));
    assert_eq!(xy.parse(b"xy"), Ok(((b'x', b'y'), 2)));
    assert_eq!(xy.parse(b"xyz"), Ok(((b'x', b'y'), 2)));
}

#[test]
fn test_or() {
    let x = satisfy(|b| b == b'x');
    let y = satisfy(|b| b == b'y');
    let mut xy = x.or(y);
    assert_eq!(xy.parse(b""), Err(()));
    assert_eq!(xy.parse(b"x"), Ok((b'x', 1)));
    assert_eq!(xy.parse(b"y"), Ok((b'y', 1)));
    assert_eq!(xy.parse(b"xy"), Ok((b'x', 1)));
    assert_eq!(xy.parse(b"yx"), Ok((b'y', 1)));
    assert_eq!(xy.parse(b"z"), Err(()));
}
