//! A/B Test Types

use crate::{AbIoLeft, AbIoRight};
use blueprint::Orbit;

/// Single Orbit
pub struct TestingCtx<O> {
    /// Orbit under test
    pub o: O,
    /// Left side of Orbit
    pub l: AbIoLeft,
    /// Right side of Orbit
    pub r: AbIoRight,
}

impl<O> TestingCtx<O>
where
    O: Orbit,
{
    #[inline]
    pub fn new(orbit: O) -> Self {
        let mut ctx = TestingCtx {
            o: orbit,
            l: AbIoLeft::default(),
            r: AbIoRight::default(),
        };

        ctx.l.out_bytes = vec![0; 16384];
        ctx.l.out_bytes_len = 0;

        ctx
    }
}
