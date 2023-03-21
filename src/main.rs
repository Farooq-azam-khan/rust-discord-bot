use std::env; 
use serenity::{
    async_trait, 
    model::channel::Message,
    prelude::*, 
    framework::standard::macros::{command, group}, 
};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General; 

struct Handler; 

#[async_trait]
impl EventHandler for Handler {} 

#[tokio::main] 
async fn main() {
    let framework = StandardFramework::new().configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP); 
    // login with a bot token from the environment 
    let token = env::var("DISCORD_TOKEN").expect("token"); 
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
                        .event_handler(Handler)
                        .framework(framework)
                        .await
                        .expect("Error Creating Client"); 

    // start listening for events with single shard 
    if let Err(why) = client.start().await {
        println!("An Error occured while running the client: {:?}", why);
    }
}

#[command] 
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?; 
    Ok(())
}
