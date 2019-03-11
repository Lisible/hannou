use serenity::command;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::env;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN")
                                 .expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("h!"))
        .cmd("addrole", add_role));

    if let Err(e) = client.start() {
        println!("An error occured while running the client: {:?}", e);
    }
}

command!(add_role(_context, message, args) {
    if args.len() != 4 {
        check_send(message.channel_id.say("Wrong number of arguments"));
        return Ok(());
    }

    let channel_id = args.single::<String>().unwrap();
    let channel_id = &channel_id[2..channel_id.len()-1].parse::<u64>()?;
    let message_id = args.single::<u64>().unwrap();
    let emoji = args.single::<String>().unwrap();
    let _role = args.single::<String>().unwrap();

    let guild = match message.guild() {
        Some(guild) => guild,
        None => {
            check_send(message.channel_id.say("Groups and DMs not supported"));
            return Ok(());
        }
    };

    let channels = match guild.read().channels() {
        Ok(channels) => channels,
        Err(e) => {
            check_send(message.channel_id.say("An error occured"));
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let channel = match channels.get(&ChannelId::from(*channel_id)) {
        Some(channel) => channel,
        None => {
            check_send(message.channel_id.say("Channel not found"));
            return Ok(());
        }
    };

    let message = match channel.message(message_id) {
        Ok(message) => message,
        Err(e) => {
            check_send(message.channel_id.say("An error occured"));
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    if let Err(e) = message.react(emoji) {
        check_send(message.channel_id.say("An error occured"));
        eprintln!("Error: {}", e);
        return Ok(());
    }

    return Ok(());
});

fn check_send(result: serenity::Result<Message>) {
    if let Err(e) = result {
        eprintln!("Error sending message: {}", e);
    }
}
