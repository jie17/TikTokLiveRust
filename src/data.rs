use std::collections::HashMap;
use std::time::Duration;

use crate::data::live_common::{HttpData, TikTokLiveSettings};

pub mod live_common;

pub fn create_default_settings(host_name: &str) -> TikTokLiveSettings {
    TikTokLiveSettings {
        language: "en-US".to_string(),
        print_logs: true,
        reconnect_on_fail: true,
        host_name: host_name.to_string(),
        http_data: HttpData {
            time_out: Duration::from_secs(3),
            cookies: create_default_cookies(),
            headers: create_default_headers(),
            params: create_default_params(),
        },
    }
}

fn create_default_params() -> HashMap<String, String> {
    let params: Vec<(&str, &str)> = vec![
        ("aid", "1988"),
        ("app_language", "en-US"),
        ("app_name", "tiktok_web"),
        ("browser_language", "en"),
        ("browser_name", "Mozilla"),
        ("browser_online", "true"),
        ("browser_platform", "Win32"),
        ("browser_version", "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36"),
        ("cookie_enabled", "true"),
        ("cursor", ""),
        ("internal_ext", ""),
        ("device_platform", "web"),
        ("focus_state", "true"),
        ("from_page", "user"),
        ("history_len", "4"),
        ("is_fullscreen", "false"),
        ("is_page_visible", "true"),
        ("did_rule", "3"),
        ("fetch_rule", "1"),
        ("identity", "audience"),
        ("last_rtt", "0"),
        ("live_id", "12"),
        ("resp_content_type", "protobuf"),
        ("screen_height", "1152"),
        ("screen_width", "2048"),
        ("tz_name", "Europe/Berlin"),
        ("referer", "https, //www.core.com/"),  
        ("root_referer", "https, //www.core.com/"),
        ("msToken", ""),
        ("version_code", "180800"),
        ("webcast_sdk_version", "1.3.0"),
        ("update_version_code", "1.3.0"),
    ];

    return params
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
}

fn create_default_headers() -> HashMap<String, String> {
    let headers: Vec<(&str, &str)> = vec![
        ("authority", "www.core.com"),
        ("Cache-Control", "max-age=0"),
        ("Accept", "text/html,application/json,application/protobuf"),
        ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36"),
        ("Referer", "https://www.tiktok.com/"),
        ("Origin", "https://www.tiktok.com"),
        ("Accept-Language", "en-US,en; q=0.9"),
    ];

    return headers
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
}

fn create_default_cookies() -> HashMap<String, String> {
    HashMap::new()
}
