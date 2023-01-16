# Luro - TODO list

A general todo list of things I wish to achieve with Luro, and where we stand. Contributions welcome.

## High Priority

Things that are pretty important to have, and hopefully will not take too much in development time

- [ ] User favs - Like the quote feature, but personalised to recall particular messages / images
    - Should only fetch NSFW saves in NSFW rooms
    - DB of `User[(nsfw, item)]`
- [ ] TODO list - Per user todo list, simple enough
- [x] Clear up the use of `.unwrap()` where possible from the codebase
- [ ] Music playlist support
- [x] Rewrite of database commands
    - [x] Add a message to the database
    - [x] Retrieve a message from the database
- [x] Guild Commands
    - [x] Get a message via the bot
    - [ ] Better formatting of activities, such as displaying Warframe stats
- [x] Hot reload support
- [ ] Interaction Database - Being able to interact with messages more expressively
    - [ ] Stickers
    - [ ] Emotes
- [ ] Message Interactions
    - [ ] Ability to reload them on bot reload
    - [ ] Role menu

## Low Priority

Things that are nice to have. Primarily a shopping list of ideas.

- [ ] Website
    - [ ] Website Landing PAge
    - [ ] Website Dashboard
- [ ] Modularity
    - [ ] Modular Database - For the moment it is hardcoded to use one provider, but this requires me to learn and understand rust's feature flags better
    - [ ] Modular Interaction Database - Allow for easier modification of emotes used in the bot
    - [ ] Modular Commands - Allow a guild owner to enable and disable subcomponents 
- [ ] Proper audit logs - Debug information and context
- [ ] Error handling improvements
- [ ] Discord support server
- [ ] A support page where users can donate / support the bot