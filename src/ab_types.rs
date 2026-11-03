//! A/B Test Types

use crate::{AbIoLeft, AbIoRight};
use blueprint::Orbit;

/// Two Fixed Orbits (Composition)
pub struct TwoTestingCtx<First, Second> {
    /// Orbit1 under test
    pub o1: First,
    /// Orbit2 under test
    pub o2: Second,
    /// Left side of Orbit
    pub l: AbIoLeft,
    /// Right side of Orbit
    pub r: AbIoRight,
}

/// Single Fixed Orbit (Standalone)
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

impl<First, Second> TwoTestingCtx<First, Second>
where
    First: Orbit,
    Second: Orbit,
{
    #[inline]
    pub fn new(orbits: (First, Second)) -> Self {
        let mut ctx = Self {
            o1: orbits.0,
            o2: orbits.1,
            l: AbIoLeft::default(),
            r: AbIoRight::default(),
        };

        ctx.l.out_bytes = vec![0; 8192];
        ctx.l.out_bytes_len = 0;
        ctx.r.out_bytes = vec![0; 8192];
        ctx.r.out_bytes_len = 0;

        ctx
    }
}
