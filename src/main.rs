use std::env; 
use dotenv::dotenv; 


use serenity::{
    async_trait, 
    model::{channel::Message, gateway::Ready}, 
    prelude::*, 
    framework::standard::macros::{command, group},
    framework::standard::{StandardFramework, CommandResult}
};

#[group]
#[commands(ping)]
struct General;

struct Handler; 

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

const HELP_MESSAGE: &str = "
Hello there, Human!
You have summoned me. Let's see about getting you what you need.
❓ Need technical help?
❓ Something wrong?
➡️ You can flag an admin with @admin
I hope that resolves your issue!
— HelpBot 🤖
";

const HELP_COMMAND: &str = "!help"; 

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why); 
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[tokio::main] 
async fn main() {
    dotenv().ok(); 
    // login with a bot token from the environment 
    let token = env::var("DISCORD_TOKEN").expect("token"); 

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    let intents = GatewayIntents::non_privileged() 
            | GatewayIntents::MESSAGE_CONTENT;
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

