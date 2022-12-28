# Notes

Misc things to keep track of.

## Dependencies

- `tracing-subscriber = "0.3"` - Used for async logs, removed as not using it for anything currently (7 added dependencies)

## .gitignore

The git ignore is configured as follows with the specified reasons

**Rust Stuff**
Build files, primarily

- `/target/` - Produced artifacts

**IDE tools**
As the name implies...

- `.fleet` - Jet brains IDE
- `.vscode` - Microsoft IDE

**Luro Specific**

Things specific to the way luro works
- `/testing/` - A folder I make locally that houses testing data
- `/data/config.toml` - General configuration for Luro
- `/data/secrets.toml` - As the name implies, main secrets for Luro
- `/data/quotes.toml` - Saved quotes
- `/data/stories.toml` - Saved stories

## Project Tree

Luro's project tree and what each folder / file is used for.