use crate::SocketError;
use futures::{stream::Stream, FutureExt};
use message_sink::{MessageSink, SinkError};
use std::{
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::{UnixSocket, UnixStream};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct SocketConnection {
    sink: MessageSink<Compat<UnixStream>>,
}

impl SocketConnection {
    pub(crate) fn new(stream: UnixStream) -> Self {
        let stream = stream.compat_write();
        let sink = MessageSink::new(stream);
        Self { sink }
    }
    pub async fn connect(path: PathBuf) -> Result<Self, std::io::Error> {
        let stream = UnixSocket::new_stream()?.connect(path).await.unwrap();
        let sink = MessageSink::new(stream.compat_write());
        Ok(Self { sink })
    }
    pub fn write(&mut self, data: Vec<u8>) -> Result<(), SocketError> {
        match self.sink.write(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(SocketError::DataCorrupt),
        }
    }
}

impl Stream for SocketConnection {
    type Item = Result<Vec<u8>, SocketError>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let socket = self.get_mut();

        match socket.sink.poll_unpin(cx) {
            Poll::Ready(Err(SinkError::Read(e))) => {
                return Poll::Ready(Some(Err(SocketError::ReadFailure(e))))
            }
            Poll::Ready(Err(SinkError::Write(e))) => {
                return Poll::Ready(Some(Err(SocketError::WriteFailure(e))))
            }
            Poll::Ready(Err(SinkError::LimitExceeded)) => {
                return Poll::Ready(Some(Err(SocketError::BufferOverflow)))
            }
            Poll::Ready(Err(SinkError::Parse(_))) => {
                return Poll::Ready(Some(Err(SocketError::DataCorrupt)))
            }
            Poll::Ready(Err(SinkError::Closed)) => return Poll::Ready(None),
            Poll::Ready(Ok(message)) => return Poll::Ready(Some(Ok(message))),
            Poll::Pending => {}
        };

        Poll::Pending
    }
}
