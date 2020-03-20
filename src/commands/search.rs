use crate::{check_msg, VoiceManager};

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    voice,
};

use log::{error, info};

#[command]
#[aliases("yt", "youtube", "y")]
fn search(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    println!();

    msg.delete(&ctx).expect("Unable to delete message.");

    let search = match args.single::<String>() {
        Ok(search) => search,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a search to a video or audio."),
            );

            return Ok(());
        }
    };

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
        let source = match voice::ytdl_search(&search) {
            Ok(source) => source,
            Err(why) => {
                error!("Error starting source: {:?}", why);

                return Ok(());
            }
        };

        handler.play_only(source);

        println!("Playing: {}", search);
        info!("Playing: {}", search);
    } else {
        println!("Not in a voice channel to play in.");
        info!("Not in a voice channel to play in.");
    }

    Ok(())
}
