use crate::{check_msg, VoiceManager};

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::{error, info};

#[command]
#[aliases("j")]
fn join(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.delete(&ctx).expect("Unable to delete message.");

    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Groups and DMs not supported."),
            );

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let channel_id = guild
        .read()
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            info!("Not in a voice channel.");

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

    if manager.join(guild_id, connect_to).is_some() {
        println!("Joined: {}", connect_to);
        info!("Joined: {}", connect_to);
    } else {
        error!("Error joining the channel.");
    }

    Ok(())
}
