@echo off
if "%1"=="" (
    echo DISCORD_BOT_TOKEN=[TOKEN] run.bat name_of_bot
    echo This is creating the ollama modelfile and running the bot.
    exit /b 1
)

set BASEDIR=%~dp0
ollama create %1 -f %BASEDIR%\modelfiles\%1.modelfile
cargo run --release %BASEDIR%\personas\%1.json
