use std::thread;
use std::time::Duration;

use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::MessageId;
use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::prelude::*;
use serenity::{prelude::EventHandler, utils::MessageBuilder};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                }
            };
            let f1 = ctx.clone();
            let sent_msg = reply_to_tx_hash(f1, &msg).await;
            if let Ok(unwrapped_msg) = sent_msg {
                thread::sleep(Duration::from_millis(4000));
                let updated_msg = update_tx_hash(ctx, msg, unwrapped_msg.id).await;
            }
            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            // let response = MessageBuilder::new()
            //     .push("User ")
            //     .push_bold_safe(&msg.author.name)
            //     .push(" used the 'ping' command in the ")
            //     .mention(&channel)
            //     .push(" channel")
            //     .build();

            // if let Err(why) = msg.channel_id.say(&context.http, &response).await {
            //     println!("Error sending message: {:?}", why);
            // }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn reply_to_tx_hash(
    ctx: Context,
    msg: &Message,
) -> Result<serenity::model::channel::Message, SerenityError> {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("").embed(|e| {
                e.title("This is a title")
                    // .description("This is a description")
                    .image("attachment://ferris_eyes.png")
                    // .fields(vec![
                    //     ("This is the first field", "This is a field body", true),
                    //     ("This is the second field", "Both fields are inline", true),
                    // ])
                    .field(
                        "This is the third field",
                        "This is not an inline field",
                        false,
                    )
                    .footer(|f| f.text("This is a footer"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .timestamp(Timestamp::now())
            })
            // .add_file("./ferris_eyes.png")
        })
        .await;

    if let Err(why) = msg {
        Err(why)
    } else {
        Ok(msg.unwrap())
    }
}

pub async fn update_tx_hash(
    ctx: Context,
    msg: Message,
    message_id: impl Into<MessageId>,
) -> Result<serenity::model::channel::Message, SerenityError> {
    let result = msg
        .channel_id
        .edit_message(ctx.http, message_id, |m| {
            m.content("Updated content").embed(|e| {
                e.title("Updated title")
                    .description("Updated description")
                    // .image("attachment://c.gif")
                    // .fields(vec![
                    //     ("This is the first field", "This is a field body", true),
                    //     ("This is the second field", "Both fields are inline", true),
                    // ])
                    .field("Update field", "Updated value", false)
                    .footer(|f| f.text("Updated footer"))
                    // Add a timestamp for the current time
                    // This also accepts a rfc3339 Timestamp
                    .timestamp(Timestamp::now())
            })
            // .attachment("./c.gif")
        })
        .await;

    if let Err(why) = result {
        Err(why)
    } else {
        Ok(result.unwrap())
    }
}
