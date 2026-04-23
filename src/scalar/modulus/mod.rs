/// Container for modular arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod64 {
    pub representative: u64,
}

/// Exact modular arithmetic.
pub mod exact;

/// Modular arithmetic based on Barrett reduction.
pub mod barrett;