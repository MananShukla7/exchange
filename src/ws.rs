use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use once_cell::sync::OnceCell;
use tokio::{io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite}, net::{TcpListener, TcpStream}};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

pub static WS_STREAM:OnceCell<WebSocketStream<TcpStream>>=OnceCell::new();

pub async fn register_name(write:&mut SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,name:String){
    let message=Message::Text(name);
    write.send(message).await.unwrap();
}

async fn handle_incoming_messages(mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>) {
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}

async fn read_and_send_messages(mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>) {
    let mut reader = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = reader.next_line().await.expect("Failed to read line") {
        println!("line: {}",line.clone());
        if !line.trim().is_empty() {
            write.send(Message::Text(line)).await.expect("Failed to send message");
        }
    }
}

#[tokio::main]
pub async fn main(){
    let listener = TcpListener::bind("0.0.0.0:6001").await.unwrap();
    while let Ok((stream,_addr)) =  listener.accept().await{
        
        let ws_stream=accept_async(stream).await.expect("Error during websocket handshake");
        // set_ws_stream(ws_stream).await;
        
    
        let (mut write,read)=ws_stream.split();
        register_name(&mut write, "RustClient".to_string()).await;
        let read_handle=tokio::task::spawn(handle_incoming_messages(read));
        let write_handle=tokio::task::spawn(read_and_send_messages(write));
        let res=tokio::join!(read_handle,write_handle);
    
    }

}

//write code to set bs stream in once cell
pub async fn set_ws_stream(ws_stream:WebSocketStream<TcpStream>){
    if WS_STREAM.get().is_none(){
    
        WS_STREAM.set(ws_stream).unwrap();
    }
}