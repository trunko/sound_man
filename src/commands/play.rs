use std::{env, fs::read_dir};

use crate::{check_msg, VoiceManager};

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    voice,
};

use log::{error, info};

#[command]
fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let sound = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a sound to a video or audio"),
            );

            return Ok(());
        }
    };

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            error!("Error finding channel info");

            return Ok(());
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");

    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        if sound.starts_with("http") {
            let source = match voice::ytdl(&sound) {
                Ok(source) => source,
                Err(why) => {
                    error!("Error loading video: {:?}", why);

                    return Ok(());
                }
            };

            handler.play(source);
        } else {
            let mut path = match env::var("SOUND_PATH") {
                Ok(path) => path,
                Err(_) => {
                    error!("No path defined for sound files");

                    return Ok(());
                }
            };

            let files = read_dir(&path).unwrap();

            for file in files {
                let file = file.unwrap();
                let file = file.file_name().into_string().unwrap();
                let file = file.split(".");
                let file: Vec<&str> = file.collect();
                info!("{}", file[0]);
            }

            path.push_str(&sound);
            path.push_str(".mp3");

            info!("{}", path);

            let source = match voice::ffmpeg(path) {
                Ok(source) => source,
                Err(why) => {
                    error!("Error playing sound: {:?}", why);

                    return Ok(());
                }
            };

            handler.play(source);
        }

        info!("Playing: {}", sound);
    } else {
        info!("Not in a voice channel to play in");
    }

    Ok(())
}
