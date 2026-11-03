//! A/B yTLS & h11spec Composition

use crate::AbStream;
use crate::TestingCtx;
use crate::*;
use crate::{AbIoLeft, AbIoRight};

use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

#[inline]
pub fn init_composition() -> (impl Orbit, impl Orbit) {
    let h11 = crate::ab_h11spec::init_server();
    let ytls = crate::ab_ytls::init_server();

    (ytls, h11)
}
