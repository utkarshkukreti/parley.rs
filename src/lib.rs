pub trait Parser {
    type Output;

    fn parse(&mut self, input: &[u8]) -> Result<(Self::Output, usize), ()>;
}
