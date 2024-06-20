use core::num::NonZeroUsize;
use thiserror::Error;

/// Denotes a potentially unbalanced block
pub type Unbalanced = Option<NonZeroUsize>;

/// Denotes a block guaranteed to be balanced
pub type Balanced = usize;

mod seal {
    pub trait Sealed {}
    
    impl Sealed for super::Unbalanced {}
    impl Sealed for super::Balanced {}
}

// Sealed trait to prevent `Block`'s state parameter abuse
/// Denotes a state type of the block
pub trait BlockState: seal::Sealed + Copy {}

impl BlockState for Unbalanced {}
impl BlockState for Balanced {}

/** A left-to-right block.
    Comes in two forms: unbalanced and balanced. */
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Block<T: BlockState> {
    opening: usize,
    closing: T
}

impl Block<Unbalanced> {
    /// Create an unbalanced block
    #[inline(always)]
    pub const fn open(idx: usize) -> Self {
        Self {
            opening: idx,
            closing: None
        }
    }
}

impl<T: BlockState> Block<T> {
    /// Force the creation of a block
    #[allow(unused_unsafe)]
    #[inline(always)]
    pub const unsafe fn new_unchecked(opening: usize, closing: T) -> Self {
        Self {
            opening,
            closing
        }
    }
    
    /// Retrieve the index of the opening token
    #[inline(always)]
    pub const fn opening(&self) -> usize {
        self.opening
    }
    
    /** Retrieve the index of the closing token.
        `Option<NonZeroUsize>` if unbalanced, `usize` otherwise */
    #[inline(always)]
    pub const fn closing(&self) -> T {
        self.closing
    }
}

/// A left-to-right block processor
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Blocks {
    inner: Vec<Block<Unbalanced>>,
    lhs: usize,
    rhs: usize
}

impl Blocks {
    /// Construct an empty `Blocks` structure
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            lhs: 0,
            rhs: 0
        }
    }
    
    /// Add a new opening token to the list
    pub fn add_left(&mut self, idx: usize) {
        self.inner.push(Block::open(idx));
        self.lhs += 1;
    }
    
    /// Add a new closing token to the list
    pub fn add_right(&mut self, idx: usize) -> Result<(), BalanceBlockError> {
        self.lhs -= 1;
        self.inner
            .get_mut(self.lhs)
            .map(|block| {
                block.closing = Some(NonZeroUsize::new(idx)?);
                Some(self.rhs += 1)
            })
            .flatten()
            .ok_or(BalanceBlockError::ExtraRight)
    }
    
    /// Check whether the tokens are balanced
    pub fn is_valid(&self) -> bool {
        self.inner.len() == self.rhs
    }
    
    /// Check the validity of the structure and return a vector of balanced blocks
    pub fn consume(self) -> Result<Vec<Block<Balanced>>, BalanceBlockError> {
        // SAFETY: the blocks are guaranteed to be balanced at this point
        self.is_valid()
            .then_some(unsafe {core::mem::transmute(self.inner)})
            .ok_or(BalanceBlockError::ExtraLeft)
    }
}

/// Balancing error
#[derive(Clone, Copy, Debug, Error)]
pub enum BalanceBlockError {
    #[error("unbalanced closing token")]
    ExtraRight,
    #[error("unbalanced opening token")]
    ExtraLeft,
}
