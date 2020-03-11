## Install
 - > cargo install systemfd cargo-watch
## launch server
 - > systemfd --no-pid -s http::5000 -- cargo run

# Routes

## Get Twitch stream
- > http://127.0.0.1:5000/dreamsbird/twitch/stream

## Get last video
- > http://127.0.0.1:5000/dreamsbird/youtube/lastvideo


##  Build + Install container
    sudo docker-compose build
    sudo docker-compose up -d

##  Connect to container
    sudo docker exec -it -u dev dreamsbird_api_php bash

## Close all container
    sudo docker stop $(sudo docker ps -a -q)