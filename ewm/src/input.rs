#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Input(u64);

#[cfg(test)]
impl Input{
    pub const DUMMY: Input = Input(42);
}
