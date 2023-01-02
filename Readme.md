# Luro - Discord Bot

Welcome to Luro, a discord bot written in rust by `Nurah#5103`.

This project has a few goals:

- A fully featured Discord bot that is not paywalled.
- A Discord bot that is modern, embracing slash commands, interactions and more with less focus on legacy 'prefix' commands.
- Reasonably simple to use, with a modest feature set.
- The ability to replace most Discord bots with one that can do it all.

## Disclaimer

This project is offered as is, with no warranty under the expectation that things will break and there will be sudden breaking changes. Once this project has got to a reasonable place, semantic versioning will be used. This project is also being used to learn git, so things may be done in strange and unexpected ways. If you have any suggestions, please do voice them!

## Getting Started

Copy ALL files, including the folders from `data/sample` to `/data`. Pretty much all of them are self explanatory. Note that any keys not specified in `secrets.toml` will cause that plugin to be disabled. If you opt to drop `bot_token`, you can pass in the environment variable `LURO_TOKEN`.

Note: There are a bunch of hard coded constants in `main.rs` which are intended to be modified by the user. These primarily relate to where config files are stored. It is recommended to store data in `XDG_DATA_HOME` on a production system. `secrets.toml` can also be specified to a different, safer path.

## Contribution and development

Refer to the document below for what is actively being worked on. Pull requests and such should be accepted, or if I did something wrong, please reach out via Discord and I'll it sorted. Note that the code has a bunch of comments with `TODO:`, which is intended to notify code functionality that needs to be worked on. This may relate to one of the project goals below, or may not.

![[TODO]]
