pub mod bitvec_incremental;
pub mod one_hot_incremental;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FindKResult {
    Found(u32),
    TimeoutReached,
}
