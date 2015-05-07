pub trait Parser {
    type Output;

    fn parse(&mut self, input: &[u8]) -> Result<(Self::Output, usize), ()>;
}

pub struct Satisfy<F: FnMut(u8) -> bool> {
    f: F
}
