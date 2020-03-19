use std::{env, fs::read_dir};

use crate::VoiceManager;

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    voice,
};

use log::{error, info};

#[command]
#[aliases("p")]
fn play(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.delete(&ctx).expect("Unable to delete message.");

    let sound = String::from(args.rest());

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            error!("Error finding channel info.");

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

            handler.play_only(source);
        } else {
            let mut path = match env::var("SOUND_PATH") {
                Ok(path) => path,
                Err(_) => {
                    error!("No path defined for sound files.");

                    return Ok(());
                }
            };

            let files = read_dir(&path).unwrap();

            for file in files {
                let file = file.unwrap();
                let file = file.file_name().into_string().unwrap();
                let file = file.split(".");
                let file: Vec<&str> = file.collect();
                if file[0].eq_ignore_ascii_case(&sound) {
                    path.push_str(file[0]);
                    path.push_str(".");
                    path.push_str(file[1]);
                }
            }

            println!("{}", path);
            info!("{}", path);

            let source = match voice::ffmpeg(path) {
                Ok(source) => source,
                Err(why) => {
                    error!("Error playing sound: {:?}", why);

                    return Ok(());
                }
            };

            handler.play_only(source);
        }

        println!("Playing: {}", sound);
        info!("Playing: {}", sound);
    } else {
        println!("Not in a voice channel to play in.");
        info!("Not in a voice channel to play in.");
    }

    Ok(())
}
