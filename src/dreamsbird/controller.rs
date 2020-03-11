use crate::dreamsbird::{Twitch, Youtube};

use std::process::Command;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::prelude::*;

impl Twitch {
    pub fn new(uid: u32, username: String, filename: String) -> Self { Self { uid, username, filename } }

    fn get_target(&self) -> String {
        dotenv().ok();
        let app_target = env::var("APP_TARGET").expect("APP_TARGET not set");
        app_target
    }
    fn make_path(&self) -> String {
        let app_target = self.get_target();
        let mut filename: String = "twitch-cli".to_owned();
        if app_target == "windows" {
            filename = "twitch-cli.exe".to_owned();
        }
        let path: String = format!("./app/{}/{}", app_target, filename);
        return path;
    }
    pub fn check_stream(&self) -> std::io::Result<()> {
        let path = self.make_path();
        let app_command = format!("isonlive-user={}", &self.uid); 
    
        let output = Command::new(path)
            .arg(app_command)
            .output()
            .unwrap();
        let encoded = String::from_utf8_lossy(output.stdout.as_slice());
        let serialized: serde_json::Value = serde_json::from_str(&encoded).unwrap();

        let mut buffer = File::create(&self.filename).unwrap();
        buffer.write_fmt(format_args!("{}", serialized)).unwrap();

        Ok(())
    }
}

impl Youtube {
    pub fn new(channel_id: String, filename: String) -> Self { Self { channel_id, filename } }

    fn get_target(&self) -> String {
        dotenv().ok();
        let app_target = env::var("APP_TARGET").expect("APP_TARGET not set");
        app_target
    }
    fn make_path(&self) -> String {
        let app_target = self.get_target();
        let mut filename: String = "youtube-cli".to_owned();
        if app_target == "windows" {
            filename = "youtube-cli.exe".to_owned();
        }
        let path: String = format!("./app/{}/{}", app_target, filename);
        return path;
    }
    pub fn check_lastvideo(&self) -> std::io::Result<()> {
        let path = self.make_path();
        let app_command = format!("lastvideo-user={}", &self.channel_id);
    
        let output = Command::new(path)
            .arg(app_command)
            .output()
            .unwrap();
        let encoded = String::from_utf8_lossy(output.stdout.as_slice());
        let serialized: serde_json::Value = serde_json::from_str(&encoded).unwrap();

        let mut buffer = File::create(&self.filename).unwrap();
        buffer.write_fmt(format_args!("{}", serialized)).unwrap();

        Ok(())
    }
}