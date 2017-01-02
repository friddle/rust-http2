extern crate http2;
extern crate futures;
extern crate tokio_core;
extern crate tokio_tls;
extern crate solicit_fork as solicit;
#[macro_use]
extern crate log;
extern crate env_logger;

use futures::Future;
use futures::Stream;

mod test_misc;

use test_misc::*;

use http2::client::*;
use http2::for_test::*;


#[test]
fn stream_count() {
    env_logger::init().unwrap();

    let server = HttpServerEcho::new();

    debug!("started server on {}", server.port);

    let client: Http2Client = Http2Client::new("::1", server.port, false).expect("connect");

    let state: ClientConnectionStateSnapshot = client.dump_state().wait().expect("state");
    assert_eq!(0, state.streams.len());

    let parts = client.start_post("/foobar", (b"xxyy"[..]).to_owned())
        .wait()
        .map(|r| r.unwrap());
    let message = SimpleHttpMessage::from_parts(parts);
    assert_eq!((b"xxyy"[..]).to_owned(), message.body);

    let state: ClientConnectionStateSnapshot = client.dump_state().wait().expect("state");
    assert_eq!(0, state.streams.len(), "{:?}", state);
}
