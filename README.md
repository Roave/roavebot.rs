# RoaveBot

At the moment, just a "hello world" bot, written with Serenity and Shuttle.

`/hello James` => `Hey there, James`

## Getting set up

Set up using [this tutorial](https://docs.shuttle.rs/templates/tutorials/discord-weather-forecast)...

```bash
cargo install cargo-shuttle
```

Populate `Secrets.toml`, e.g.:

```toml
DISCORD_TOKEN = '<token>'
DISCORD_GUILD_ID = '<server ID>'
```

## Running

```bash
cargo shuttle run
```

## Deploying

```bash
cargo shuttle deploy
```
