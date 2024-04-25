use crate::SocketConnection;
use crate::SocketError;
use futures::stream::Stream;
use std::{
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::UnixListener;

pub struct SocketServer {
    listener: UnixListener,
}

impl SocketServer {
    pub fn listen(path: PathBuf) -> Result<Self, std::io::Error> {
        let listener = UnixListener::bind(path)?;
        Ok(Self { listener })
    }
}

impl Stream for SocketServer {
    type Item = Result<SocketConnection, SocketError>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let server = self.get_mut();
        match server.listener.poll_accept(cx) {
            Poll::Ready(Ok((stream, _))) => {
                let conn = SocketConnection::new(stream);
                return Poll::Ready(Some(Ok(conn)));
            }
            Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(SocketError::AcceptFailure(e)))),
            Poll::Pending => {}
        };
        Poll::Pending
    }
}
