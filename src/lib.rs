pub mod u8;
pub mod combinator;

pub use combinator::{Then, Or, Map};

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

    fn map<O, F>(self, f: F) -> Map<O, F, Self>
        where F: FnMut(Self::Output) -> O
    {
        Map {
            p: self,
            f: f
        }
    }
}
