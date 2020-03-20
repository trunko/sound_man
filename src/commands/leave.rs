use crate::{check_msg, VoiceManager};

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::{error, info};

#[command]
#[aliases("l")]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!();

    msg.delete(&ctx).expect("Unable to delete message.");

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
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        println!("Left voice channel.");
        info!("Left voice channel.");
    } else {
        error!("Not in a voice channel.");
    }

    Ok(())
}
