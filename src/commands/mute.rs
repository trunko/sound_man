use crate::{check_msg, VoiceManager};

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::info;

#[command]
fn mute(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Groups and DMs not supported"),
            );

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

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            info!("Not in a voice channel");

            return Ok(());
        }
    };

    if handler.self_mute {
        info!("Already muted");
    } else {
        handler.mute(true);

        info!("Now muted");
    }

    Ok(())
}