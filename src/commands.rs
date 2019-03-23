/*
* MIT License
*
* Copyright (c) 2019 Clément SIBILLE
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use serenity::command;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::id::RoleId;
use crate::database;

command!(add_role(_context, message, args) {
    if args.len() != 4 {
        check_send(message.channel_id.say("Wrong number of arguments"));
        return Ok(());
    }

    let channel_id = args.single::<String>().unwrap();
    let channel_id = (&channel_id[2..channel_id.len()-1]).parse::<u64>()?;
    let message_id = args.single::<u64>().unwrap();
    let emoji = args.single::<String>().unwrap();
    let role_name = args.single::<String>().unwrap();
    let role_name = &role_name[1..role_name.len()];

    let guild = match message.guild() {
        Some(guild) => guild,
        None => {
            check_send(message.channel_id.say("Groups and DMs not supported"));
            return Ok(());
        }
    };

    let role = match guild.read().role_by_name(&role_name) {
        Some(r) => r.clone(),
        None => {
            check_send(message.channel_id.say("Role not found"));
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

    let channel = match channels.get(&ChannelId::from(channel_id)) {
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

    if let Err(e) = message.react(emoji.clone()) {
        check_send(message.channel_id.say("An error occured"));
        eprintln!("Error: {}", e);
        return Ok(());
    }

    database::add_role(channel_id, message_id, emoji, *RoleId::from(role).as_u64());
    return Ok(());
});

fn check_send(result: serenity::Result<Message>) {
    if let Err(e) = result {
        eprintln!("Error sending message: {}", e);
    }
}
