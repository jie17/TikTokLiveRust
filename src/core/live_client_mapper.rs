use tokio::sync::mpsc;

use crate::generated::events::{TikTokLiveEvent, TikTokWebsocketResponseEvent};
use crate::generated::messages::webcast::WebcastResponse;

#[derive(Clone)]
pub struct TikTokLiveMessageMapper {}

impl TikTokLiveMessageMapper {
    pub async fn handle_webcast_response(
        &self,
        webcast_response: WebcastResponse,
        event_sender: &mpsc::Sender<TikTokLiveEvent>,
    ) {
        let _ = event_sender
            .send(TikTokLiveEvent::OnWebsocketResponse(
                TikTokWebsocketResponseEvent {
                    raw_data: webcast_response.clone(),
                },
            ))
            .await;
        for message in &webcast_response.messages {
            self.handle_single_message(message.clone(), event_sender)
                .await;
        }
    }
}
