use futures_util::{SinkExt, StreamExt};
use log::warn;
use protobuf::Message;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::Request;

use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::generated::events::{TikTokConnectedEvent, TikTokDisconnectedEvent, TikTokLiveEvent};
use crate::generated::messages::webcast::{WebcastPushFrame, WebcastResponse};
use crate::http::http_data::LiveConnectionDataResponse;

pub struct TikTokLiveWebsocketClient {
    pub(crate) message_mapper: TikTokLiveMessageMapper,
    pub(crate) running: Arc<AtomicBool>,
    pub(crate) join_set: JoinSet<()>,
    pub(crate) event_sender: mpsc::Sender<TikTokLiveEvent>,
}

impl TikTokLiveWebsocketClient {
    pub fn new(
        message_mapper: TikTokLiveMessageMapper,
        event_sender: mpsc::Sender<TikTokLiveEvent>,
    ) -> Self {
        TikTokLiveWebsocketClient {
            message_mapper,
            running: Arc::new(AtomicBool::new(false)),
            join_set: JoinSet::new(),
            event_sender,
        }
    }

    pub async fn start(
        &mut self,
        response: LiveConnectionDataResponse,
    ) -> Result<(), anyhow::Error> {
        let host = response
            .web_socket_url
            .host_str()
            .expect("Invalid host in WebSocket URL");

        let request = Request::builder()
            .method("GET")
            .uri(response.web_socket_url.to_string())
            .header("Host", host)
            .header("Upgrade", "websocket")
            .header("Connection", "upgrade")
            .header("Sec-Websocket-Key", "asd")
            .header("Cookie", response.web_socket_cookies)
            .header("Sec-Websocket-Version", "13")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36")
            .body(())?;

        let (socket, _) = connect_async(request).await?;
        let (mut write, mut read) = socket.split();
        let (tx, mut rx) = mpsc::channel::<tungstenite::protocol::Message>(100);
        let tx2 = tx.clone();

        let _ = self
            .event_sender
            .send(TikTokLiveEvent::OnConnected(TikTokConnectedEvent {}))
            .await;

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let message_mapper = self.message_mapper.clone();

        let event_sender = self.event_sender.clone();

        self.join_set.spawn(async move {
            loop {
                let message = rx.recv().await;
                if message.is_none() {
                    warn!("Unable to read message, exiting");
                    break;
                }
                let message = message.unwrap();
                if let Err(err) = write.send(message).await {
                    warn!("Unable to send message, {}", err);
                }
            }
        });

        self.join_set.spawn(async move {
            loop {
                let message = tungstenite::protocol::Message::binary(vec![
                    //3A026862
                    0x3A, 0x02, 0x68, 0x62,
                ]);
                if let Err(err) = tx.send(message).await {
                    warn!("Unable to send ping message, {}", err);
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        });

        let running = self.running.clone();
        self.join_set.spawn(async move {
            while running.load(Ordering::SeqCst) {
                let optional_message = read.next().await;

                if optional_message.is_none() {
                    warn!("WS connection closed, exiting");
                    event_sender
                        .send(TikTokLiveEvent::OnDisconnected(TikTokDisconnectedEvent {}))
                        .await
                        .unwrap();
                    break;
                }
                let result_message = optional_message.unwrap();
                if result_message.is_err() {
                    warn!("Unable to read message, {}", result_message.err().unwrap());
                    break;
                }
                let message = result_message.unwrap();

                let buffer = message.into_data();

                let push_frame = WebcastPushFrame::parse_from_bytes(buffer.as_slice());
                if push_frame.is_err() {
                    warn!("Unable to read push frame, {}", push_frame.err().unwrap());
                    continue;
                }
                let mut unwrapped_push_frame = push_frame.unwrap();
                let webcast_response =
                    WebcastResponse::parse_from_bytes(unwrapped_push_frame.Payload.as_mut_slice());
                if webcast_response.is_err() {
                    warn!(
                        "Unable to read webcast response, {}",
                        webcast_response.err().unwrap()
                    );
                    continue;
                }
                let unwrapped_webcast_response = webcast_response.unwrap();

                if unwrapped_webcast_response.needsAck {
                    let mut push_frame_ack = WebcastPushFrame::new();
                    push_frame_ack.PayloadType = "ack".to_string();
                    push_frame_ack.LogId = unwrapped_push_frame.LogId;
                    push_frame_ack.Payload =
                        unwrapped_webcast_response.internalExt.clone().into_bytes();

                    let binary = push_frame_ack.write_to_bytes().unwrap();
                    let message = tungstenite::protocol::Message::binary(binary);
                    if let Err(err) = tx2.send(message).await {
                        warn!("Unable to send ping message, {}", err);
                    }
                }

                message_mapper
                    .handle_webcast_response(unwrapped_webcast_response, &event_sender)
                    .await;
            }
        });
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.join_set.abort_all();
    }
}
