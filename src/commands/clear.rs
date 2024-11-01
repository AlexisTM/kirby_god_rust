use std::sync::Arc;

use serenity::all::{CommandInteraction, Context, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;
use serenity::prelude::RwLock;

use crate::persona::Persona;

pub async fn run(ctx: &Context, command: &CommandInteraction, persona: Arc<RwLock<Persona>>) {
    let history_id = command.channel_id.to_string();
    persona.write().await.clear(&history_id);
    if let Err(why) = command
        .create_response(
            &ctx.http,
            serenity::all::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("I forgot all about us... Hope you miss me now"),
            ),
        )
        .await
    {
        println!("Cannot respond to slash command: {why}");
    };
}

pub fn register() -> CreateCommand {
    CreateCommand::new("clear").description("Reset my memory.")
}
