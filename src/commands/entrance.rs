use serenity::client::Context;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use std::{env, fs, path::Path};

use log::{error, info};

#[command]
fn entrance(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    println!();

    msg.delete(&ctx).expect("Unable to delete message.");

    let sound = args.rest().to_string();
    let entrance_path = "config/entrance/".to_string();
    let user_id = msg.author.id.as_u64();

    if !Path::new(&entrance_path).exists() {
        match fs::create_dir(&entrance_path) {
            Err(why) => {
                println!("Could not create entrance file directory.");
                error!("Could not create entrance file directory: {:?}", why.kind());
                return Ok(());
            }
            Ok(_) => {}
        }
    }

    let mut dest: String = format!("{}{}", entrance_path, user_id);

    if !sound.trim().is_empty() {
        let mut source = match env::var("SOUND_PATH") {
            Ok(source) => source,
            Err(_) => {
                println!("No path defined for sound files.");
                error!("No path defined for sound files.");

                return Ok(());
            }
        };

        let files = fs::read_dir(&source).unwrap();

        for file in files {
            let file = file.unwrap();
            let file = file.file_name().into_string().unwrap();
            let file = file.split(".");
            let file: Vec<&str> = file.collect();
            if file[0].eq_ignore_ascii_case(&sound) {
                source.push_str(file[0]);
                source.push_str(".");
                source.push_str(file[1]);

                dest.push_str(".");
                dest.push_str(file[1]);
            }
        }

        match fs::copy(&source, &dest) {
            Ok(_) => {
                println!("Set entrance sound for user: {}", msg.author.name);
                info!("Set entrance sound for user: {}", msg.author.name);
            }
            Err(_) => {
                println!("Error copying entrance sound for user: {}", msg.author.name);
                error!("Error copying entrance sound for user: {}", msg.author.name);
            }
        }
    } else {
        let files = fs::read_dir(&entrance_path).unwrap();
        let mut found = false;

        for file in files {
            let file = file.unwrap();
            let file = file.file_name().into_string().unwrap();
            let file = file.split(".");
            let file: Vec<&str> = file.collect();
            if file[0].to_string() == user_id.to_string() {
                dest.push_str(".");
                dest.push_str(file[1]);
                found = true;
            }
        }

        if found {
            match fs::remove_file(&dest) {
                Ok(_) => {
                    println!("Set entrance sound for user: {}", msg.author.name);
                    info!("Set entrance sound for user: {}", msg.author.name);
                }
                Err(_) => {
                    println!("Error copying entrance sound for user: {}", msg.author.name);
                    error!("Error copying entrance sound for user: {}", msg.author.name);
                }
            }
        }
    }

    Ok(())
}
