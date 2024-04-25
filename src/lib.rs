mod connection;
mod error;
mod server;

pub use connection::SocketConnection;
pub use error::SocketError;
pub use server::SocketServer;

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use temp_dir::TempDir;

    #[tokio::test]
    async fn it_works() {
        let dir = TempDir::new().unwrap();
        let socket = dir.path().join("foobar.socket");
        let mut server = SocketServer::listen(socket.clone()).unwrap();
        tokio::task::spawn(async move {
            let mut client = SocketConnection::connect(socket).await.unwrap();
            loop {
                let msg = client.next().await.unwrap().unwrap();
                let msg = std::str::from_utf8(&msg).unwrap();
                assert_eq!(msg, "ping");
                client.write("pong".into()).unwrap();
                client.write("pong".into()).unwrap();
            }
        });
        let mut conn = server.next().await.unwrap().unwrap();
        let mut i = 0;
        loop {
            i += 1;
            conn.write("ping".into()).unwrap();
            conn.write("ping".into()).unwrap();
            let msg = conn.next().await.unwrap().unwrap();
            let msg = std::str::from_utf8(&msg).unwrap();
            assert_eq!(msg, "pong");
            if i % 1000 == 0 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn multiple_clients() {
        let dir = TempDir::new().unwrap();
        let socket = dir.path().join("foobar.socket");
        let mut server = SocketServer::listen(socket.clone()).unwrap();
        for _ in 0..10 {
            let socket = socket.clone();
            tokio::task::spawn(async move {
                let mut client = SocketConnection::connect(socket).await.unwrap();
                loop {
                    let msg = client.next().await.unwrap().unwrap();
                    let msg = std::str::from_utf8(&msg).unwrap();
                    assert_eq!(msg, "ping");
                    client.write("pong".into()).unwrap();
                }
            });
        }
        async fn handle_conn(mut conn: SocketConnection) {
            let mut i = 0;
            loop {
                i += 1;
                conn.write("ping".into()).unwrap();
                let msg = conn.next().await.unwrap().unwrap();
                let msg = std::str::from_utf8(&msg).unwrap();
                assert_eq!(msg, "pong");
                if i % 10 == 0 {
                    break;
                }
            }
        }
        let mut conn = Vec::new();
        for _ in 0..10 {
            conn.push(server.next().await.unwrap().unwrap());
        }
        tokio::join!(
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
            handle_conn(conn.pop().unwrap()),
        );
    }
}
