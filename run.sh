#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo """DISCORD_BOT_TOKEN=[TOKEN] ./run.sh name_of_bot
This is creating the ollama modelfile and running the bot.
"""
    exit 1
fi

BASEDIR=$(dirname $0)
ollama create ${1} -f ${BASEDIR}/modelfiles/${1}.modelfile
cargo run --release ${BASEDIR}/personas/${1}.json

