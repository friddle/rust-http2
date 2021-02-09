#![deny(intra_doc_link_resolution_failure)]
// TODO: add docs
//#![deny(missing_docs)]

//! Asynchnous HTTP/2 client and server implementation.
//!
//! Based on futures/tokio.
//!
//! This crate is used to implement [`grpc` crate](https://github.com/stepancheg/grpc-rust),
//! and probably not usable for anything else.

#[macro_use]
extern crate log;

mod solicit;

mod error;
mod result;

mod client;
mod codec;
mod server;

mod ascii;

mod client_died_error_holder;
mod common;

mod data_or_headers;
mod data_or_headers_with_flag;
mod data_or_trailers;
mod message;

mod futures_misc;

mod headers_place;
mod req_resp;

mod assert_types;

mod hpack;
mod solicit_async;
mod solicit_misc;

mod display_comma_separated;
mod misc;

mod resp;

mod exec;

mod log_ndc_future;

pub(crate) mod net;

pub(crate) mod bytes_ext;

pub use crate::net::addr::AnySocketAddr;

pub use crate::solicit::error_code::ErrorCode;
pub use crate::solicit::header::name::HeaderName;
pub use crate::solicit::header::name::PseudoHeaderName;
pub use crate::solicit::header::value::HeaderValue;
pub use crate::solicit::header::Header;
pub use crate::solicit::header::Headers;
pub use crate::solicit::stream_id::StreamId;
pub use crate::solicit::HttpScheme;
pub use crate::solicit::end_stream::EndStream;

pub use crate::client::conf::ClientConf;
pub use crate::client::req::ClientRequest;
pub use crate::client::tls::ClientTlsOption;
pub use crate::client::Client;
pub use crate::client::ClientBuilder;
pub use crate::client::ClientInterface;
pub use crate::common::sender::SendError;
pub use crate::common::sender::SenderState;
pub use crate::common::window_size::StreamDead;

pub use crate::server::conf::ServerAlpn;
pub use crate::server::conf::ServerConf;
pub use crate::server::handler::ServerHandler;
pub use crate::server::handler::ServerHandlerContext;
pub use crate::server::handler_paths::ServerHandlerPaths;
pub use crate::server::increase_in_window::ServerIncreaseInWindow;
pub use crate::server::req::ServerRequest;
pub use crate::server::resp::ServerResponse;
pub use crate::server::stream_handler::ServerRequestStreamHandler;
pub use crate::server::tls::ServerTlsOption;
pub use crate::server::Server;
pub use crate::server::ServerBuilder;

pub use crate::data_or_trailers::DataOrTrailers;
pub use crate::data_or_trailers::HttpStreamAfterHeaders;
pub use crate::resp::Response;

pub use crate::message::SimpleHttpMessage;

pub use crate::error::Error;
pub use crate::result::Result;

pub use bytes_ext::buf_get_bytes::BufGetBytes;
pub use bytes_ext::bytes_deque::BytesDeque;

/// Functions used in tests
#[doc(hidden)]
pub mod for_test {
    pub use crate::common::conn::ConnStateSnapshot;
    pub use crate::common::stream::HttpStreamStateSnapshot;
    pub use crate::server::conn::ServerConn;
    pub use crate::solicit_async::recv_raw_frame_sync;

    pub use crate::solicit::frame::HttpSettings;
    pub use crate::solicit::window_size::WindowSize;
    pub use crate::solicit::DEFAULT_SETTINGS;

    pub mod solicit {
        pub use crate::solicit::*;
    }
    pub mod hpack {
        pub use crate::hpack::*;
    }
}
