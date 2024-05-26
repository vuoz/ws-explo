use hyper::Request;
use hyper::*;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:5000/ws";
    let uri: Uri = url.parse().unwrap();
    println!("Please enter you key!");
    let mut key = String::new();
    loop {
        match std::io::stdin().read_line(&mut key) {
            Ok(_) => (),
            Err(_) => println!("Error reading your input!"),
        };
        if!key.is_empty() {
            break;
        }
    }
    // for windows
    let key = key.replace("\r\n", "");
    // for linux
    let key = key.replace("\n", "");
    println!("Got key!");

    let request = Request::builder()
        .uri(uri)
        .header("Sec-WebSocket-key", "")
        .header("Sec-WebSocket-key", "")
        .header("Host", "localhost")
        .header("Connection", "upgrade")
        .header("upgrade", "Websocket")
        .header("Sec-WebSocket-version", "13")
        .header("x-key", key)
        .body(())
        .unwrap();
    let (mut ws, _) = tokio_tungstenite::tungstenite::client::connect(request).unwrap();
    println!("Successfully connected to Server");

    //This is just the ping from the Server
    let _ = match ws.read() {
        Ok(msg) => msg,
        Err(e) => return println!("Cannot read {}", e),
    };
    loop {
        let msg = match ws.read() {
            Ok(msg) => msg,
            Err(e) => match e {
                tokio_tungstenite::tungstenite::Error::Io(error) => {
                    let std::io::Error { .. } =  error;
                    return println!("Server closed the connection!");
                },
                e => return println!("Error reading from ws: {}", e),
            },
        };
        println!("Got Message: {}", msg)
    }
}
