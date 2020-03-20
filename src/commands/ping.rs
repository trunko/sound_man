use crate::check_msg;

use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use log::info;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!();

    msg.delete(&ctx).expect("Unable to delete message.");

    println!("Ping!");
    info!("Ping!");
    check_msg(msg.channel_id.say(&ctx.http, "Pong!"));

    Ok(())
}
