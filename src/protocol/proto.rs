extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use self::bytes::BytesMut;
use self::tokio_io::codec::{Encoder, Decoder};
use self::tokio_proto::pipeline::ServerProto;
use self::tokio_io::{AsyncRead, AsyncWrite};
use self::tokio_io::codec::Framed;

use super::codec::Line as LineCodec;

pub struct Line;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Line {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = String;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}