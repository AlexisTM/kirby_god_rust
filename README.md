Ollama Persona Discord bot
==================

This project allows you to run your **custom** Ollama persona **locally** as a Discord bot. It can:
- Mimick a virtual character (see Mavin, Kirby or Pastafari)
- Provide mental and relationship help (see Samantha), `This doesn't replace professional help`
- Python code teacher
- Dungeon Master assitant to generate encounters on the flight 

Quickstart
=============

```
# Use WSL on Windows, skip on Linux/MacOS
wsl --install Ubuntu-24.04
wsl

# Install Ollama and pre-pull mistral
curl -fsSL https://ollama.com/install.sh | sh
ollama pull mistral

# Start your bot
git clone https://github.com/AlexisTM/ollama-discord-persona-rs
cd ollama-discord-persona-rs
DISCORD_BOT_TOKEN=DISCORD_BOT_TOKEN ./run.sh marvin

# or manually
ollama create marvin -f modelfiles/marvin.modelfile
DISCORD_BOT_TOKEN=DISCORD_BOT_TOKEN cargo run --release modelfiles/marvin.json
```

Commands
=============

- `Direct message`: The persona replies to the message
- `/botname prompt`: Slash command to contact a specific persona
- `/clear`: Slash command to remove the persona memory

Custom bot
===============

To make your custom persona, create a modelfile like [marvin.modelfile](modelfiles/marvin.modelfile) (see the [Modelfile format](https://github.com/ollama/ollama/blob/main/docs/modelfile.md)) and prepare your network. To get your modelfile started, use `ollama show [modelname] --modelfile`

Then, create a json file with the botname, the model you just created and optional extra generation options to overwrite the PARAMETER you set in the modelfile ([Options available](https://github.com/pepperoni21/ollama-rs/blob/5d6cd76aa4bf073a037a43a4eff70310f07654cd/src/generation/options.rs#L5-L22))

```json
{
    "botname": "Marvin",
    "model": "marvin",
    "options": {
        "temperature": 0.5,
    }
}
```

You can then run the persona as:

```bash
ollama create marvin -f modelfiles/marvin.modelfile
DISCORD_BOT_TOKEN=DISCORD_BOT_TOKEN cargo run --release modelfiles/marvin.json
```

> The botname should not have special characters nor spaces right now, to be compatible for the /slash command.

FAQ
========

Missing pkg-config on WSL
--------------

```bash
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
```

Technical help on how to make a Discord bot:
==================

Create a bot application: https://discordpy.readthedocs.io/en/stable/discord.html

Configure intents for your bot: https://discordpy.readthedocs.io/en/stable/intents.html

In the oauth section of discord dev portal, make a link to allow your bot to join your server such as:

https://discord.com/api/oauth2/authorize?client_id=APPID&permissions=2215115840&scope=bot

In this case, we only need the bot scope and READ/WRITE messages permissions
