//! A/B h11spec

use crate::AbStream;
use crate::TestingCtx;
use crate::*;
use crate::{AbIoLeft, AbIoRight};

use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

use blueprint_h11spec::H11Serving;
use blueprint_h11spec::H11SpecServer;
use yaws_app_rick_roll::RickRoll;

pub const RICK: &[u8] = include_bytes!("/tmp/rick-roll.gif");
pub const RICK_SIZE: usize = RICK.len();

#[inline]
pub fn init_server() -> impl Orbit {
    H11Serving::with_app(RickRoll::with_rick(RICK))
}
