//! A/B TLS & H11Spec Composition Testing driver

pub use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

use crate::AbStream;
use crate::TwoTestingCtx;
use crate::{AbIoLeft, AbIoRight};

use openssl::ssl::{SslConnector, SslMethod, SslStream};

#[test]
fn init_c_ytls_h11spec() {
    let mut _test_ctx = TwoTestingCtx::new(crate::ab_c_ytls_h11spec::init_composition());
}

#[test]
fn init_c_rustls_h11spec() {
    let mut _test_ctx = TwoTestingCtx::new(crate::ab_c_rustls_h11spec::init_composition());
}

use crate::ExpectedFailure;

/// Trigger a Rick Roll testing with composition
#[inline]
pub fn c_rickroll(ctx: (impl Orbit, impl Orbit), exp_failure: Option<ExpectedFailure>) {
    let test_ctx = TwoTestingCtx::new(ctx);

    let mut openssl_build = SslConnector::builder(SslMethod::tls()).unwrap();

    openssl_build.set_ca_file(crate::CA).unwrap();

    let openssl = openssl_build.build();

    use crate::AbStreamForward;

    let client =
        AbStream::with_read_injector(test_ctx, crate::ab_two_injector::ab_read_injector_two);

    let hostname = match exp_failure {
        Some(ExpectedFailure::ServerCertVerifyFailed) => "here.we.fail",
        _ => "test.rustcryp.to",
    };

    use openssl::ssl::HandshakeError;
    let mut ssl_stream = match openssl.connect(hostname, client) {
        Ok(s) => s,
        Err(HandshakeError::SetupFailure(e)) => panic!("Setup Failure: {:?}", e),
        Err(HandshakeError::Failure(mid_handshake)) => {
            panic!("Failure mid-handshake: {:?}", mid_handshake.into_error())
        }
        Err(HandshakeError::WouldBlock(mid_handshake)) => {
            panic!("WouldBlock mid-handshake {:?}", mid_handshake.into_error())
        }
    };

    //************************************************
    // Application layer
    //************************************************
    use std::io::{Read, Write};
    ssl_stream
        .write_all(b"GET / HTTP/1.1\r\nHost: test.rustcryp.to\r\n\r\n")
        .unwrap();

    let mut app_recvd: Vec<u8> = vec![0; 8192];
    let bytes_in = ssl_stream.read(&mut app_recvd).unwrap();

    let response = core::str::from_utf8(&app_recvd[..49]);
    let expected_response = "HTTP/1.1 200 OK\r\nContent-Length: 461688\r\n\r\nGIF89a";

    assert_eq!(response, Ok(expected_response), "unexpected response");

    let mut rick_body: Vec<u8> = Vec::with_capacity(461688);

    rick_body.extend(&app_recvd[43..bytes_in]);

    let consumed_expect = rick_body.capacity();

    loop {
        let mut app_recvd: Vec<u8> = vec![0; 8192];
        let bytes_in = ssl_stream.read(&mut app_recvd).unwrap();

        if rick_body.len() + bytes_in > consumed_expect {
            panic!(
                "Overflow of {} bytes on expected response read.",
                (rick_body.len() + bytes_in) - consumed_expect
            );
        }

        rick_body.extend(&app_recvd[0..bytes_in]);

        if rick_body.len() == consumed_expect {
            break;
        }
    }
}

#[test]
fn c_rickroll_ytls() {
    c_rickroll(crate::ab_c_ytls_h11spec::init_composition(), None);
}

#[test]
fn c_rickroll_rustls() {
    c_rickroll(crate::ab_c_rustls_h11spec::init_composition(), None);
}
