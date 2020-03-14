use crate::paranoi4k::{Twitch, Youtube};

use std::{thread, time};

pub fn main_twitch(uid: u32, username: String, tickrate: u64) {
    let streamer = Twitch::new(uid, username, "paranoi4k_twitch.json".to_owned());

    loop {
        let duration = time::Duration::from_secs(tickrate);
        streamer.check_stream().unwrap();
        thread::sleep(duration);
    }
}

pub fn main_youtube(channel_id: String, tickrate: u64) {
    let youtuber = Youtube::new(channel_id, "paranoi4k_youtube.json".to_owned());

    loop {
        let duration = time::Duration::from_secs(tickrate);
        youtuber.check_lastvideo().unwrap();
        thread::sleep(duration);
    }
}