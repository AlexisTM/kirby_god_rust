use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, EditInteractionResponse, ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::prelude::RwLock;

use crate::god::God;

pub async fn run(ctx: &Context, command: &CommandInteraction, god: Arc<RwLock<God>>) {
    let author_name = command.user.name.clone();
    if let Some(ResolvedOption {
        value: ResolvedValue::String(prompt_slice),
        ..
    }) = command.data.options().first()
    {
        let _ = command.defer(&ctx.http).await;
        let prompt = { god.read().await.get_prompt(&author_name, prompt_slice) };
        let response = { god.read().await.brain.request(&prompt).await };
        if let Some(response) = response {
            let content = format!(
                "{}\n\n{}: {}",
                &prompt.last().unwrap().content,
                god.read().await.get_botname(),
                response.content.clone()
            );
            let builder = EditInteractionResponse::new().content(content);
            if let Err(why) = command.edit_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            } else {
                god.write().await.set_prompt_response(
                    &author_name,
                    prompt_slice,
                    &response.content,
                );
            }
        } else {
            println!("Error with ollama");
        }
    } else {
        println!("No prompt provided.");
    }
}

pub fn register(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .description("Speak to this bot.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "request",
                "The message to your favourite bot.",
            )
            .required(true),
        )
}
