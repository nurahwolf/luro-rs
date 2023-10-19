# Luro - Discord Bot v2

Welcome to Luro, a discord bot written in rust by `Nurah#5103`.

This project has a few goals:

- A fully featured Discord bot that is not paywalled.
- A Discord bot that is modern, embracing slash commands, interactions and more with less focus on legacy 'prefix' commands.
- Reasonably simple to use, with a modest feature set.
- The ability to replace most Discord bots with one that can do it all.

## Project Crates

- **luro-database** - A standard way to get information from several database drivers. Primary driver uses `sqlx` and connects to progress, with WIP / testing drivers for using `toml` files and `diesel`.
- **luro-dice** - A simple dice roller library, used for the D&D commands. Can be used entirely standalone.
- **luro-framework** - A wrapper around the `twilight` crate, allowing for standardised access of data
- **luro-model** - Models and utility consumed throughout the rest of Luro. Can also be standalone if needed.
- **luro-twilight** - Where the commands and events live, uses the database driver and `luro-framework` to operate.
- **twilight-interactions** - Out of tree copy of `twilight-interactions`, just updated to work on the master branch of twilight, which this bot uses.

## IMPORTANT!!

This is a complete project REBASE on the [Twilight](https://github.com/twilight-rs/twilight) library, instead of Serenity. This primarily comes down to more flexibility (and more things to learn), as the Poise and Serenity framework are not super well integrated. That, and the serenity branches seem to get out of sync a lot so it was difficult to actually get a project working.

As a bonus, there are a bunch fewer libs and so the end binary is smaller. Yay!

**Music Note**  
While I was planning to keep the project pure rust, this branch uses [LavaLink](https://github.com/freyacodes/Lavalink) for audio. I was originally planning to use Songbird, but due to it repeatedly trying to pull in old version of Twilight, I eventually gave up and went the path of least resistance.

**Database Note**  
I'm unsure on the end result for the database, but I'm potentially considering `neo4j` considering Discord data fits a graph quite well. Any recommendations are welcome, though an embedded / semi-embedded DB would be my ideal pick.

## Disclaimer

This project is also being used to learn git, so things may be done in strange and unexpected ways. Once this project has got to a reasonable place, semantic versioning will be used. If you have any suggestions, please do voice them!

ADDITIONALLY, this branch may become abandoned, or replace mainline Luro! I'm not sure yet as this is effectively a trial to see what lib works best for Luro's needs.

## Getting Started

Unlike the mainline branch of Luro, this project heavily uses environment variables. In the future I hope to have this use both config and env vars, so they user may choose what strategy works best for them.

As the project current stands, the best way to run is as follows:

- Follow [LavaLink](https://github.com/freyacodes/Lavalink)'s getting started, or use these steps
  - Grab the latest release jar, such as [v3.7.4](https://github.com/freyacodes/Lavalink/releases/download/3.7.4/Lavalink.jar) (Direct download link)
  - Copy said jar to the folder `Lavalink`, with the filename `Lavalink.jar`
  - Change directory to the `Lavalink` folder and start up the server, with something like the following: `java -jar Lavalink.jar`
  - `Lavalink` should generate `application.yml`. Modify it to contain AT LEAST the following:
  ```yaml
  server: # REST and WS server
    port: 6969 # My port of choice for... Reasons.
    address: 127.0.0.1 # Listening on localhost is probably preferred, but you can set it to 0.0.0.0 if you wish to host Lavalink on another server. 
  lavalink:
    server:
      password: "a_super_mega_secure_password_that_is_in_plaintext"
  ```
  - Rerun `java -jar Lavalink.jar`, the port should change to whatever you defined.
  - Now declare environment variables. You can use an env file, or via the shell: `export DISCORD_TOKEN=mydiscordtoken && export LAVALINK_HOST=127.0.0.1:1234 && export LAVALINK_AUTHORIZATION=myauthkey && echo "Luro + Lavalink env set"`
  - Run luro: `rust +nightly run` or `cargo +nightly run`

Make sure you have nightly, if you don't it can be grabbed via `rustup toolchain install nightly`.

Unlike mainline Luro, `cmake` is NOT required, since the `Lavalink` jar contains all dependencies.

## Contribution and development

Refer to the document below for what is actively being worked on. Pull requests and such should be accepted, or if I did something wrong, please reach out via Discord and I'll it sorted. Note that the code has a bunch of comments with `TODO:`, which is intended to notify code functionality that needs to be worked on. This may relate to one of the project goals below, or may not.

[[TODO]]