use anyhow::Context as _;
use serenity::all::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{info};

struct Bot {
    discord_guild_id: GuildId
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // We are creating a vector with commands
        // and registering them on the server with the guild ID we have set.
        let commands = vec![
            CreateCommand::new("hello")
                .description("Say hello")
                .add_option(
                    CreateCommandOption::new(
                        serenity::all::CommandOptionType::String,
                        "person",
                        "Person to say hello to",
                    )
                    .required(true)
                ),
            CreateCommand::new("source")
                .description("Find where the source code lives..."),
        ];
        let commands = &self
            .discord_guild_id
            .set_commands(&ctx.http, commands)
            .await
            .unwrap();

        info!("Registered commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => {
                    let person = command.data.options.iter().find(|opt| opt.name == "person").clone().unwrap().value.as_str().unwrap();
                    format!("Hey there, {}", person)
                },
                "source" => "My source code is located at https://github.com/Roave/roavebot.rs <:uwugrim:1131146511024144394>".to_owned(),
                command => unreachable!("Unknown command: {}", command),
            };

            let data = CreateInteractionResponseMessage::new().content(response_content);
            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let discord_guild_id = secrets
        .get("DISCORD_GUILD_ID")
        .context("'DISCORD_GUILD_ID' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::empty();

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            discord_guild_id: GuildId::new(discord_guild_id.parse().unwrap())
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
