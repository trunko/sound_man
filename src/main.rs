//! Requires the "cache", "methods", and "voice" features be enabled in your
//! Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["cache", "framework", "standard_framework", "voice"]
//! ```
mod commands;

extern crate log;
extern crate log4rs;

use std::{env, sync::Arc};

// Import the client's bridge to the voice manager. Since voice is a standalone
// feature, it's not as ergonomic to work with as it could be. The client
// provides a clean bridged integration with voice.
use serenity::client::bridge::voice::ClientVoiceManager;

// Import the `Context` from the client and `parking_lot`'s `Mutex`.
//
// `parking_lot` offers much more efficient implementations of `std::sync`'s
// types. You can read more about it here:
//
// <https://github.com/Amanieu/parking_lot#features>
use serenity::{client::Context, prelude::Mutex};

use serenity::{
    client::{Client, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    model::{channel::Message, gateway::Ready},
    Result as SerenityResult,
};

// This imports `typemap`'s `Key` as `TypeMapKey`.
use serenity::prelude::*;

use log::{error, info};

use commands::{join::*, leave::*, ping::*, play::*, search::*, stop::*};

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as: {}", ready.user.name);
        info!("Connected as: {}", ready.user.name);
    }
}

group!({
    name: "general",
    options: {},
    commands: [join, leave, play, ping, search, stop]
});

fn main() {
    kankyo::init().expect("Unable to load .env file.");

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");

    let prefix = env::var("PREFIX").expect("Expected a prefix for commands in the environment.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&prefix))
            .group(&GENERAL_GROUP),
    );

    let _ = client
        .start()
        .map_err(|why| error!("Client ended: {:?}", why));
}

/// Checks that a message successfully sent; if not, then logs why.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        error!("Error sending message: {:?}", why);
    }
}
