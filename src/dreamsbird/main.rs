use crate::dreamsbird::{Twitch, Youtube};

use std::{thread, time};

pub fn main_twitch(uid: u32, username: String, tickrate: u64) {
    let streamer = Twitch::new(uid, username, "dreamsbird_twitch.json".to_owned());

    loop {
        let duration = time::Duration::from_secs(tickrate);
        streamer.check_stream().unwrap();
        thread::sleep(duration);
    }
}

pub fn main_youtube(channel_id: String, tickrate: u64) {
    let youtuber = Youtube::new(channel_id, "dreamsbird_youtube.json".to_owned());

    loop {
        let duration = time::Duration::from_secs(tickrate);
        youtuber.check_lastvideo().unwrap();
        thread::sleep(duration);
    }
}