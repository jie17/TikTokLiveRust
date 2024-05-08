use std::cell::Cell;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use log::{error, info, warn};

use crate::core::live_client_events::TikTokLiveEventObserver;
use crate::core::live_client_http::TikTokLiveHttpClient;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::core::live_client_websocket::TikTokLiveWebsocketClient;
use crate::data::live_common::ConnectionState::{CONNECTED, CONNECTING, DISCONNECTED};
use crate::data::live_common::{ConnectionState, TikTokLiveInfo, TikTokLiveSettings};
use crate::generated::events::TikTokLiveEvent;
use crate::http::http_data::LiveStatus::HostOnline;
use crate::http::http_data::{LiveConnectionDataRequest, LiveDataRequest, LiveUserDataRequest};
use tokio::sync::mpsc;

pub struct TikTokLiveClient {
    settings: TikTokLiveSettings,
    http_client: TikTokLiveHttpClient,
    event_observer: TikTokLiveEventObserver,
    websocket_client: Arc<TikTokLiveWebsocketClient>,
    room_info: TikTokLiveInfo,
    event_sender: mpsc::Sender<TikTokLiveEvent>,
}

impl TikTokLiveClient {
    pub(crate) fn new(
        settings: TikTokLiveSettings,
        http_client: TikTokLiveHttpClient,
        event_observer: TikTokLiveEventObserver,
        websocket_client: TikTokLiveWebsocketClient,
        room_info: TikTokLiveInfo,
        event_sender: mpsc::Sender<TikTokLiveEvent>,
    ) -> Self {
        return TikTokLiveClient {
            settings,
            http_client,
            event_observer,
            websocket_client: Arc::new(websocket_client),
            room_info,
            event_sender,
        };
    }

    pub async fn connect(self) -> Result<(), Box<dyn std::error::Error>> {
        if *(self.room_info.connection_state.lock().unwrap()) != DISCONNECTED {
            warn!("Client already connected!");
            return Err("Client already connected!".into());
        }

        self.set_connection_state(CONNECTING);

        info!("Getting live user information's");
        let response = self
            .http_client
            .fetch_live_user_data(LiveUserDataRequest {
                user_name: self.settings.host_name.clone(),
            })
            .await;

        info!("Getting live room information's");
        let room_id = response.room_id;
        let response = self
            .http_client
            .fetch_live_data(LiveDataRequest {
                room_id: room_id.clone(),
            })
            .await;
        if response.live_status != HostOnline {
            error!(
                "Live stream for host is not online!, current status {:?}",
                response.live_status
            );
            return Err("Live stream for host is not online!".into());
        }

        info!("Getting live connections information's");
        let response = self
            .http_client
            .fetch_live_connection_data(LiveConnectionDataRequest {
                room_id: room_id.clone(),
            })
            .await;

        let client_arc = Arc::new(self);

        let ws = TikTokLiveWebsocketClient {
            message_mapper: TikTokLiveMessageMapper {},
            running: Arc::new(AtomicBool::new(false)),
        };
        ws.start(response, client_arc).await;
        Ok(())
    }

    pub fn disconnect(&self) {
        self.websocket_client.stop();
        self.set_connection_state(DISCONNECTED)
    }

    pub fn publish_event(&self, event: TikTokLiveEvent) {
        let sender = self.event_sender.clone();
        tokio::spawn(async move {
            if let Err(e) = sender.send(event).await {
                eprintln!("Error sending event: {}", e);
            }
        });
    }

    pub fn set_connection_state(&self, state: ConnectionState) {
        let mut data = self.room_info.connection_state.lock().unwrap();
        *data = state;
        info!("TikTokLive: {:?}", *data);
    }
}

impl Drop for TikTokLiveClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}
