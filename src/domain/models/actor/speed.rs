// TODO: error[E0658]: discriminants on non-unit variants are experimental
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Speed {
    Slow = 2,
    Normal = 4,
    Fast = 8,
}
