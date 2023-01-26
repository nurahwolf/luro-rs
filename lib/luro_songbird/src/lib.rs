pub mod commands;
mod nowplaying;
mod joinvoice;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc
};

use poise::{
    async_trait,
    serenity_prelude::{ChannelId, Guild, Http, User}
};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use crate::nowplaying::now_playing;

pub struct TrackStartNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
    pub accent_colour: [u8; 3],
    pub guild: Guild,
    pub user: User
}

#[async_trait]
impl VoiceEventHandler for TrackStartNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let metadata = track_list.last().unwrap().1.metadata();
            self.chan_id
                .send_message(&self.http, |builder| {
                    builder.embed(|embed| {
                        *embed = now_playing(self.accent_colour, self.guild.clone(), Some(self.user.clone()), metadata);
                        embed
                    })
                })
                .await
                .ok();
        }

        None
    }
}

pub struct ChannelDurationNotifier {
    pub chan_id: ChannelId,
    pub count: Arc<AtomicUsize>,
    pub http: Arc<Http>
}

#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let count_before = self.count.fetch_add(1, Ordering::Relaxed);
        self.chan_id
            .say(
                &self.http,
                &format!("I've been in this channel for {} minutes!", count_before + 1)
            )
            .await
            .ok();

        None
    }
}

pub struct SongEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.chan_id.say(&self.http, "Song faded out completely!").await.ok();

        None
    }
}

pub struct SongFader {
    pub chan_id: ChannelId,
    pub http: Arc<Http>
}

#[async_trait]
impl VoiceEventHandler for SongFader {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(state.volume / 2.0);

            if state.volume < 1e-2 {
                let _ = track.stop();
                self.chan_id.say(&self.http, "Stopping song...").await.ok();
                Some(Event::Cancel)
            } else {
                self.chan_id.say(&self.http, "Volume reduced.").await.ok();
                None
            }
        } else {
            None
        }
    }
}
