local-socket
============

Simple `client` / `server` wrapper for a `tokio` backed [unix domnain socket](https://en.wikipedia.org/wiki/Unix_domain_socket).

There is no need to roll your own message framing. Under the hood, this handles message framing when sending byte arrays over the wire.

When a client sends a byte array to the server - and vice-versa - the server is guarenteed to receive a single event on the other side with the complete array.

# Usage

Server:

```
let dir = TempDir::new().unwrap();
let socket = dir.path().join("foobar.socket");

// Create a new server listening at a filesystem path
let mut server = SocketServer::listen(socket).unwrap();

loop {

  // Handle inbound connections
  let mut connection = server.next().await.unwrap().unwrap();

  // Receive a message from the client
  let msg = connection.next().await.unwrap().unwrap();

  // Send the client a message
  connection.write("pong".into()).unwrap();
}
```

Client:

```
let mut connection = SocketConnection::connect(socket).await.unwrap();

// Ping - Pong
loop {
  // Send the server a message
  connection.write("ping".into()).unwrap();

  // Receive a message from the server
  let msg = connection.next().await.unwrap().unwrap();
}
```
