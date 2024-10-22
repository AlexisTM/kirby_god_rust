use clap::Parser;

pub mod commands;
pub mod god;
pub mod ollama;

use god::{God, GodConfig, GodNursery};

use serenity::all::Interaction;
use serenity::gateway::ActivityData;
use std::env;
use std::fs;
use std::{collections::HashMap, sync::Arc};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler, GatewayIntents, RwLock},
};

struct Handler {}

#[allow(dead_code)]
fn get_name<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

async fn get_or_create_bot(ctx: &Context, key: u64) -> Arc<RwLock<God>> {
    let data = ctx.data.read().await;
    let nursery = data
        .get::<GodNursery>()
        .expect("There should be a nursery here.");

    let default_god = data
        .get::<GodConfig>()
        .expect("There should be a default config in the context.");

    let has_bot = nursery.read().await.contains_key(&key);

    if !has_bot {
        let new_god = God::from_config(default_god.clone());
        let mut write_nursery = nursery.write().await;
        write_nursery.insert(key, Arc::new(RwLock::new(new_god)));
    }

    let bot = {
        let read_nursery = nursery.read().await;
        read_nursery.get(&key).unwrap().clone()
    };

    bot
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let key = command.channel_id;
            let god = get_or_create_bot(&ctx, key.into()).await;
            let botname = god.read().await.get_botname();

            match command.data.name.as_str() {
                "clear" => commands::clear::run(&ctx, &command, god).await,
                data => {
                    if data == botname.to_lowercase() {
                        commands::chat::run(&ctx, &command, god).await
                    } else {
                        println!("not implemented :(")
                    }
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // This is only used for private messages
        if msg.guild_id == None {
            return;
        }

        // Prevent answering itself.
        let bot_user = ctx.http.get_current_user().await;
        let val = match bot_user {
            Ok(val) => Some(val.id),
            Err(_) => None,
        };

        if val.is_none() || val.unwrap() == msg.author.id {
            return;
        }

        let key = msg.channel_id;

        let prompt_slice = &msg.content[..];
        let author_name = msg.author.name.clone();
        let god = get_or_create_bot(&ctx, key.into()).await;

        let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };

        let response = { god.read().await.brain.request(&prompt).await };
        if let Some(response) = response {
            if let Err(why) = msg.channel_id.say(&ctx.http, &response.content).await {
                println!("Error sending message: {:?}", why);
            }
            {
                god.write().await.set_prompt_response(
                    &author_name,
                    prompt_slice,
                    &response.content,
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        use serenity::model::user::OnlineStatus;
        let activity = ActivityData::watching("the world burn");
        let status = OnlineStatus::DoNotDisturb;
        ctx.set_presence(Some(activity), status);

        let data = ctx.data.read().await;
        let config = data
            .get::<GodConfig>()
            .expect("There should be god configuration.");

        let guild_commands = ctx
            .http
            .create_global_commands(&vec![
                commands::chat::register(&config.botname.to_ascii_lowercase()),
                commands::clear::register(),
            ])
            .await;

        match guild_commands {
            Ok(_) => println!("Chat guild command added."),
            Err(why) => println!("Failed to add the guild command: {:?}", why),
        }
    }
}

#[derive(Parser)]
struct Cli {
    pub god: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token_discord =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a DISCORD_BOT_TOKEN in the environment");

    let args = Cli::parse();

    println!("Reading: {:?}", args.god);

    let god_data: String = fs::read_to_string(&args.god)
        .unwrap_or_else(|_| panic!("The god {:?} file must be readable.", &args.god));
    let config = match serde_json::from_str::<GodConfig>(&god_data) {
        Ok(config) => Some(config),
        Err(err) => {
            println!("Parsing failed: {err}");
            None
        }
    }
    .expect("The god config should be valid.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_INTEGRATIONS;

    let mut client = Client::builder(&token_discord, intents)
        .event_handler(Handler {})
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GodConfig>(config);
        data.insert::<GodNursery>(RwLock::new(HashMap::default()));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
