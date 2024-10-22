use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, EditInteractionResponse,
    ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::prelude::RwLock;

use crate::persona::Persona;

pub async fn run(ctx: &Context, command: &CommandInteraction, persona: Arc<RwLock<Persona>>) {
    let author_name = if let Some(global_name) = &command.user.global_name {
        global_name.clone()
    } else {
        command.user.name.clone()
    };
    if let Some(ResolvedOption {
        value: ResolvedValue::String(prompt_slice),
        ..
    }) = command.data.options().first()
    {
        let _ = command.defer(&ctx.http).await;
        let prompt = { persona.read().await.get_prompt(&author_name, prompt_slice) };
        let response = { persona.read().await.brain.request(&prompt).await };
        if let Some(response) = response {
            let content = format!(
                "\nFrom **{author_name}:**```{prompt_slice}```**{}:**```{}```",
                persona.read().await.get_botname(),
                response.content,
            );
            let builder = EditInteractionResponse::new().content(content);
            if let Err(why) = command.edit_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            } else {
                persona.write().await.set_prompt_response(
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
