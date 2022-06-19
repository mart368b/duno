use futures_util::{SinkExt, StreamExt};
use poem::web::websocket::{Message, WebSocket};
use poem::web::{Data, Path};
use poem::{handler, IntoResponse};

#[handler]
pub fn ws(
    Path(name): Path<String>,
    ws: WebSocket,
    sender: Data<&tokio::sync::broadcast::Sender<String>>
) -> impl IntoResponse {
    let sender = sender.clone();
    let mut receiver = sender.subscribe();

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    if sender.send(format!("{}: {}", name, text)).is_err() {
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                if sink.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });
    })
}
