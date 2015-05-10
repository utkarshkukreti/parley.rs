use Parser;

pub struct Then<P1: Parser, P2: Parser> {
    pub p1: P1,
    pub p2: P2
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
    pub p1: P1,
    pub p2: P2
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

pub struct Map<O, F: FnMut(P::Output) -> O, P: Parser> {
    pub p: P,
    pub f: F
}

impl<O, F, P> Parser for Map<O, F, P>
    where F: FnMut(P::Output) -> O, P: Parser
{
    type Output = O;
    fn parse(&mut self, input: &[u8]) -> Result<(O, usize), ()> {
        self.p.parse(input).map(|(r, c)| ((self.f)(r), c))
    }
}

pub struct Many<P: Parser> {
    pub p: P
}

impl<P>Parser for Many<P>
    where P: Parser
{
    type Output = Vec<P::Output>;
    fn parse(&mut self, input: &[u8]) -> Result<(Vec<P::Output>, usize), ()> {
        let mut parsed = vec![];
        let mut consumed = 0;
        loop {
            match self.p.parse(&input[consumed..]) {
                Ok((r, c)) => {
                    parsed.push(r);
                    consumed += c;
                },
                Err(()) => break
            }
        }
        Ok((parsed, consumed))
    }
}

#[test]
fn test_then() {
    use u8::satisfy;

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
    use u8::satisfy;

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

#[test]
fn test_map() {
    use u8::satisfy;

    let mut digit = satisfy(|b| b'0' <= b && b <= b'9').map(|b| b - b'0');

    assert_eq!(digit.parse(b""), Err(()));
    assert_eq!(digit.parse(b"0"), Ok((0, 1)));
    assert_eq!(digit.parse(b"9"), Ok((9, 1)));
    assert_eq!(digit.parse(b"z"), Err(()));
}

#[test]
fn test_many() {
    use u8::satisfy;

    let mut digits = satisfy(|b| b'0' <= b && b <= b'9').many();

    assert_eq!(digits.parse(b""), Ok((vec![], 0)));
    assert_eq!(digits.parse(b"0"), Ok((vec![b'0'], 1)));
    assert_eq!(digits.parse(b"0z"), Ok((vec![b'0'], 1)));
    assert_eq!(digits.parse(b"01"), Ok((vec![b'0', b'1'], 2)));
    assert_eq!(digits.parse(b"01z"), Ok((vec![b'0', b'1'], 2)));
    assert_eq!(digits.parse(b"z"), Ok((vec![], 0)));
}
