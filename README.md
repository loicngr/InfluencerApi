## Install
 - > cargo install systemfd cargo-watch
## launch server
 - > systemfd --no-pid -s http::5000 -- cargo run

# Routes
## Get Twitch stream
- > http://127.0.0.1:5000/dreamsbird/twitch/stream
## Get last video
- > http://127.0.0.1:5000/dreamsbird/youtube/lastvideo

## More informations : 
[twitch-cli.exe](https://github.com/loicngr/RustTwitchCli) and [youtube-cli.exe](https://github.com/loicngr/RustYoutubeCli) app here.
