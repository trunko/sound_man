use crate::VoiceManager;

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::{error, info};

#[command]
#[aliases("s", "skip")]
fn stop(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.delete(&ctx).expect("Unable to delete message.");

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
        println!("Stopping current sound.");
        info!("Stopping current sound.");
        handler.stop();
    } else {
        println!("Unable to stop sound.");
        info!("Unable to stop sound.");
    }

    Ok(())
}
