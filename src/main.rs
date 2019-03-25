use std::env;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

use hannou::commands::*;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN")
                                 .expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("h!"))
        .cmd("addrole", add_role));

    println!("Client started!");
    if let Err(e) = client.start() {
        println!("An error occured while running the client: {:?}", e);
    }
}


