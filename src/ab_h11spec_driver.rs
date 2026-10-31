//! A/B TLS Testing driver

pub use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

use crate::AbStream;
use crate::TestingCtx;
use crate::{AbIoLeft, AbIoRight};

use crate::ab_h11spec::RICK;
use crate::ab_h11spec::RICK_SIZE;

#[test]
fn init_h11spec() {
    let mut _test_ctx = TestingCtx::new(crate::ab_h11spec::init_server());
}

use crate::ExpectedFailure;

/// Trigger rick testing
/// Trigger a teapot testing
pub fn h11rick(ctx: impl Orbit, exp_failure: Option<ExpectedFailure>) {
    let mut test_ctx = TestingCtx::new(ctx);

    struct NothingBurger;

    let input = "GET / HTTP/1.1\r\nHost: unsuspecting.one\r\n\r\n".as_bytes();
    let copied = test_ctx.l.copy_in(&input);
    assert_eq!(copied, input.len());

    // Spin the Rick's fortune wheel with what we have!
    let _pos = test_ctx
        .o
        .advance_with(&mut NothingBurger, &mut test_ctx.l, &mut NoRight);

    assert_eq!(test_ctx.l.out_bytes_len, 16319);

    // Because we know where the HTTP Body starts we can just cheat and get the minimal HTTP response out from first spin
    let start = String::from_utf8_lossy(&test_ctx.l.out_bytes[0..43]);
    // We should have Rick's response back for the HTTP Layer as follows

    const EXPECT_RESP: &'static str = "HTTP/1.1 200 OK\r\nContent-Length: 461688\r\n\r\n";

    assert_eq!(EXPECT_RESP, start);

    let mut rick_in: Vec<u8> = Vec::with_capacity(RICK_SIZE);
    rick_in.extend(&test_ctx.l.out_bytes[43..test_ctx.l.out_bytes_len]);
    test_ctx.l.out_bytes = vec![0; 16319];
    test_ctx.l.out_bytes_len = 0;

    // Now let's get rest of the Rick we expect..
    loop {
        let _pos = test_ctx
            .o
            .advance_with(&mut NothingBurger, &mut test_ctx.l, &mut NoRight);

        if test_ctx.l.out_bytes_len == 0 {
            break;
        }

        rick_in.extend(&test_ctx.l.out_bytes[0..test_ctx.l.out_bytes_len]);

        test_ctx.l.out_bytes = vec![0; 16319];
        test_ctx.l.out_bytes_len = 0;
    }

    assert_eq!(rick_in.len(), RICK_SIZE);
}

#[test]
fn rick_h11spec() {
    h11rick(crate::ab_h11spec::init_server(), None);
}
