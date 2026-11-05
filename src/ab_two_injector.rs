//! A/B Injector injects into std Read call to spin the state machine in between

use crate::AbStreamForward;
use crate::TwoTestingCtx;
use blueprint::{Left, NoRight, Orbit, Right};

pub(crate) fn ab_read_injector_two<First: Orbit, Second: Orbit>(
    u: &mut TwoTestingCtx<First, Second>,
    to_client: &mut [u8],
    to_server: &mut [u8],
) -> AbStreamForward {
    // Process network layer / TLS data from Left side
    u.l.copy_in(to_server);

    struct User;

    if u.l.out_bytes_len == 0 {
        let r = u.o1.advance_with(&mut User, &mut u.l, &mut u.r);
    }

    let (l_pending_in, l_pending_out) = u.l.left_lens();

    if to_client.len() == 0 {
        panic!("No RX headroom for read.");
    }

    // Process application layer data from Right side and send back to Left
    // Canon way is for App to consume Left (mirrored Right by I/O runtime)
    // but here we have a special case just direct from Right for testing
    let (r_pending_in, _) = u.r.right_lens();

    if r_pending_in > 0 {
        let app_buf_in = &u.r.in_bytes[..r_pending_in];
        //println!("App: {}", core::str::from_utf8(app_buf_in).unwrap());

        match core::str::from_utf8(app_buf_in) {
            Ok("GET / HTTP/1.1\r\nHost: test.rustcryp.to\r\n\r\n") => {}
            _ => panic!(
                "Wrong cleartext data received: {:?}",
                core::str::from_utf8(app_buf_in)
            ),
        }
    }

    let l_pre_lens = u.l.left_lens();
    let r_pre_lens = u.r.right_lens();

    if u.l.is_ready() && l_pre_lens.1 == 0 {
        // We reflect first layer Rigth side as Left for the second layer
        let mut right_as_left = u.r.impl_left_mut();
        let r_as_l_lens = right_as_left.left_lens();

        let _ =
            u.o2.advance_with(&mut User, &mut right_as_left, &mut NoRight);
        drop(right_as_left);

        let l_lens = u.l.left_lens();
        let r_lens = u.r.right_lens();

        if l_lens.1 == 0 {
            let r = u.o1.advance_with(&mut User, &mut u.l, &mut u.r);
            u.r.all_sent_right_out();
        }
    }

    let (l_pending_in, l_pending_out) = u.l.left_lens();
    if l_pending_out > 0 {
        let copy_len = if to_client.len() < l_pending_out {
            to_client.len()
        } else {
            l_pending_out
        };
        to_client[..copy_len].copy_from_slice(&u.l.out_bytes[..copy_len]);
        let new_len = l_pending_out - copy_len;
        u.l.left_set_lens(l_pending_in, new_len);

        return AbStreamForward {
            discard_write: to_server.len(),
            read_status: Ok(copy_len),
        };
    }
    AbStreamForward {
        discard_write: to_server.len(),
        read_status: Ok(0),
    }
}
