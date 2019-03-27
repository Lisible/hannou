use std::env;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

use hannou::commands::*;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    let pool = mysql::Pool::new("mysql://root:root@hannou-db:3306/hannou").unwrap();
    pool.prep_exec(r"CREATE TABLE reaction_roles (
                        guild_id varchar(18) NOT NULL,
                        channel_id varchar(18) NOT NULL,
                        message_id varchar(18) NOT NULL,
                        emoji varchar(255) NOT NULL,
                        role_id varchar(18) NOT NULL,
                        PRIMARY KEY (guild_id, channel_id, message_id, emoji)
                     )", ()).unwrap();


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


