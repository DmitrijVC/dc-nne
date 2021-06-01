use std::env;
use std::str::FromStr;
use serenity::{
    async_trait,
    framework::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const EMOTE_SIZES: [u8; 4] = [32u8, 40u8, 64u8, 128u8];


struct Handler {
    emote_size: u8
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        let client_id = ctx.cache.current_user_id().await.0;

        if msg.author.id == client_id {
            if msg.content.starts_with(":") && msg.content.ends_with(":") {
                let guilds = ctx.cache.guilds().await;

                for guild_id in guilds.into_iter() {
                    let emojis = ctx.http.get_emojis(guild_id.0).await.unwrap();
                    for emoji in emojis.into_iter() {

                        if format!(":{}:", emoji.name).to_lowercase() == msg.content.to_lowercase() {
                            msg.edit(&ctx.http, |m| {
                                let size = if self.emote_size == 0u8 {
                                    String::new()
                                } else {
                                    format!("?size={}", self.emote_size)
                                };

                                m.content(format!(
                                    "{}{}",
                                    emoji.url(),
                                    size
                                ))
                            }).await.unwrap()
                        };
                    };
                };
            };
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Client [{}] is now connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("USAGE: [<EMOTE_SIZE>]\n\t[optional] EMOTE_SIZES: 32, 40, 64, 128");
        return;
    }

    let token = if let Ok(token) = env::var("NNE_USER_TOKEN") {
        token
    } else {
        println!("Missing environment variable [NNE_USER_TOKEN]!");
        return;
    };

    let size = if args.get(1).is_some() {
        let size_str = args.get(1).unwrap();
         match u8::from_str(size_str) {
            Ok(val) => {
                if EMOTE_SIZES.contains(&val) {
                    val
                } else {
                    0u8
                }
            },
            Err(_) => 0u8,
        }
    } else {
        0u8
    };

    let mut client = Client::builder(&token)
        .event_handler(
            Handler{
                emote_size: size
            }
        )
        .framework(StandardFramework::new())
        .await
        .expect("Error while creating a client!");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
