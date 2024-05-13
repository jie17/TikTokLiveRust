use anyhow::anyhow;
use log::error;
use serde_json::Value;

use crate::http::http_data::LiveStatus::{HostNotFound, HostOffline, HostOnline};
use crate::http::http_data::UserStatus::{Live, LivePaused, NotFound, Offline};
use crate::http::http_data::{LiveDataResponse, LiveUserDataResponse, SignServerResponse};

pub fn map_live_user_data_response(json: String) -> Result<LiveUserDataResponse, anyhow::Error> {
    let json_value: Value = serde_json::from_str(json.as_str())?;

    let message = json_value["message"]
        .as_str()
        .ok_or(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;
    if message.eq("params_error") {
        error!("fetchRoomIdFromTiktokApi -> Unable to fetch roomID, contact the developer");
        return Err(anyhow!(
            "fetchRoomIdFromTiktokApi -> Unable to fetch roomID, contact the developer"
        ))?;
    }
    if message.eq("user_not_found") {
        error!("TikTokUserInfo.UserStatus.NotFound");
        return Err(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;
    }

    let option_data = json_value["data"].as_object();
    let option = option_data.ok_or(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;
    let user = option["user"]
        .as_object()
        .ok_or(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;
    let room_id = user["roomId"]
        .as_str()
        .ok_or(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;
    let status = user["status"]
        .as_i64()
        .ok_or(anyhow!("TikTokUserInfo.UserStatus.NotFound"))?;

    let user_status = match status {
        2 => Live,
        3 => LivePaused,
        4 => Offline,
        _ => NotFound,
    };

    let live_room = option["liveRoom"]
        .as_object()
        .ok_or(anyhow!("no liveRoom"))?;
    let start_time = live_room["startTime"]
        .as_i64()
        .ok_or(anyhow!("no startTime"))?;

    Ok(LiveUserDataResponse {
        user_status,
        json: json.to_string(),
        room_id: room_id.to_string(),
        started_at_timestamp: start_time,
    })
}

pub fn map_live_data_response(json: String) -> LiveDataResponse {
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();

    let data = json_value["data"].as_object().unwrap();

    let status = data["status"].as_i64().unwrap();
    let title = data["title"].as_str().unwrap();
    let viewers = data["user_count"].as_i64().unwrap();
    let live_status = match status {
        2 => HostOnline,
        4 => HostOffline,
        _ => HostNotFound,
    };

    let stats = data["stats"].as_object().unwrap();
    let likes = stats["like_count"].as_i64().unwrap();
    let total_viewers = stats["total_user"].as_i64().unwrap();

    LiveDataResponse {
        json,
        live_status,
        total_viewers,
        viewers,
        likes,
        title: title.to_string(),
    }
}

pub fn map_sign_server_response(json: String) -> SignServerResponse {
    let json_value: Value = serde_json::from_str(json.as_str()).unwrap();
    let signed_url = json_value["signedUrl"].as_str().unwrap();
    let user_agent = json_value["User-Agent"].as_str().unwrap();

    SignServerResponse {
        signed_url: signed_url.to_string(),
        user_agent: user_agent.to_string(),
    }
}
