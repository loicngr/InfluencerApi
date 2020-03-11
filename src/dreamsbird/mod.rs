mod model;
mod routes;
mod controller;
mod main;

pub use model::{Twitch, Youtube};
pub use routes::init_routes;
pub use main::{main_twitch, main_youtube};