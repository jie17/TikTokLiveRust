use crate::core::live_client::TikTokLiveClient;
use crate::generated::events::TikTokLiveEvent;

pub type TikTokEventHandler = fn(client: &TikTokLiveClient, event: &TikTokLiveEvent);

#[derive(Clone)]
pub struct TikTokLiveEventObserver
{
    pub events: Vec<TikTokEventHandler>,
}

impl Default for TikTokLiveEventObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl TikTokLiveEventObserver
{
    pub fn new() -> Self
    {
        TikTokLiveEventObserver
        {
            events: vec![]
        }
    }

    pub fn subscribe(&mut self, handler: TikTokEventHandler)
    {
        self.events.push(handler);
    }

    pub fn publish(&self, client: &TikTokLiveClient, event: TikTokLiveEvent)
    {
        for handler in &self.events
        {
            handler(client, &event);
        }
    }
}
