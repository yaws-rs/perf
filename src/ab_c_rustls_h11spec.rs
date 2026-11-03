//! A/B RusTLS & h11spec Composition

use crate::AbStream;
use crate::TestingCtx;
use crate::*;
use crate::{AbIoLeft, AbIoRight};

use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

#[inline]
pub fn init_composition() -> (impl Orbit, impl Orbit) {
    let h11 = crate::ab_h11spec::init_server();
    let rustls = crate::ab_rustls::init_server();

    (rustls, h11)
}
