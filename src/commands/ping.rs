use crate::check_msg;

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::info;

#[command]
fn ping(context: &mut Context, msg: &Message) -> CommandResult {
    info!("Ping!");
    check_msg(msg.channel_id.say(&context.http, "Pong!"));

    Ok(())
}
