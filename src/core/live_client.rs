use anyhow::anyhow;
use log::{error, info, warn};

use crate::core::live_client_http::TikTokLiveHttpClient;
use crate::core::live_client_websocket::TikTokLiveWebsocketClient;
use crate::data::live_common::ConnectionState::{CONNECTED, CONNECTING, DISCONNECTED};
use crate::data::live_common::{ConnectionState, TikTokLiveInfo, TikTokLiveSettings};
use crate::http::http_data::LiveStatus::HostOnline;
use crate::http::http_data::{LiveConnectionDataRequest, LiveDataRequest, LiveUserDataRequest};

pub struct TikTokLiveClient {
    settings: TikTokLiveSettings,
    http_client: TikTokLiveHttpClient,
    websocket_client: TikTokLiveWebsocketClient,
    room_info: TikTokLiveInfo,
}

impl TikTokLiveClient {
    pub(crate) fn new(
        settings: TikTokLiveSettings,
        http_client: TikTokLiveHttpClient,
        websocket_client: TikTokLiveWebsocketClient,
        room_info: TikTokLiveInfo,
    ) -> Self {
        TikTokLiveClient {
            settings,
            http_client,
            websocket_client,
            room_info,
        }
    }

    pub async fn connect(&self) -> Result<(), anyhow::Error> {
        if *(self.room_info.connection_state.lock().unwrap()) != DISCONNECTED {
            warn!("Client already connected!");
            return Err(anyhow!("Client already connected!"));
        }

        self.set_connection_state(CONNECTING);

        info!("Getting live user information's");
        let response = self
            .http_client
            .fetch_live_user_data(LiveUserDataRequest {
                user_name: self.settings.host_name.clone(),
            })
            .await?;

        info!("Getting live room information's");
        let room_id = response.room_id;
        let response = self
            .http_client
            .fetch_live_data(LiveDataRequest {
                room_id: room_id.clone(),
            })
            .await?;
        if response.live_status != HostOnline {
            error!(
                "Live stream for host is not online!, current status {:?}",
                response.live_status
            );
            return Err(anyhow!("Live stream for host is not online!"));
        }

        info!("Getting live connections information's");
        let response = self
            .http_client
            .fetch_live_connection_data(LiveConnectionDataRequest {
                room_id: room_id.clone(),
            })
            .await?;

        self.websocket_client.start(response).await?;
        self.set_connection_state(CONNECTED);
        Ok(())
    }

    pub fn disconnect(&self) {
        self.websocket_client.stop();
        self.set_connection_state(DISCONNECTED)
    }

    pub fn set_connection_state(&self, state: ConnectionState) {
        let mut data = self.room_info.connection_state.lock().unwrap();
        *data = state;
        info!("TikTokLive: {:?}", *data);
    }
}
