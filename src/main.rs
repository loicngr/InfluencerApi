#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::{env, thread};

mod paranoi4k;
mod api_error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let youtube_channel_id = env::var("YOUTUBE_CHANNEL_ID").expect("YOUTUBE_CHANNEL_ID not set");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| 
        App::new()
            .configure(paranoi4k::init_routes)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    thread::spawn(move || {
        paranoi4k::main_twitch( 39116516 , "paranoi4ktv".to_owned(), 120);
    });
    thread::spawn(move || {
        paranoi4k::main_youtube(youtube_channel_id, 2400);
    });
    
    info!("Starting server");
    server.run().await
}
