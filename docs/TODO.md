# Luro - TODO list

A general todo list of things I wish to achieve with Luro, and where we stand. Contributions welcome.

## High Priority

Things that are pretty important to have, and hopefully will not take too much in development time

- [ ] Clear up the use of `.unwrap()` where possible from the codebase
- [ ] Music playlist support
- [ ] Rewrite of database commands
    - [ ] Add a message to the database
    - [ ] Retrieve a message from the database
- [ ] Guild Commands
    - [ ] Get a message via the bot
    - [ ] Better formatting of activities, such as displaying Warframe stats
- [ ] Hot reload support
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