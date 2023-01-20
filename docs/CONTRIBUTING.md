# How to contribute

Thank you for taking your interest in contributing to Luro! Here are some steps to get started.
If you have not already, it is probably easiest to talk to Luro's main developer, Nurah, on Discord at `Nurah#5103`.


Here are some important resources:

- [Secure Development and Deployment Guidelines](https://www.ncsc.gov.uk/collection/developers-collection) - Provided by the NCSC and a good stepping stone for developing Luro securely.
- [Secure Development Practices](https://www.ncsc.gov.uk/collection/developers-collection/principles) - NCSC's secure development principles.

## Submitting changes

Please send a [GitHub Pull Request to Luro](https://github.com/nurahwolf/luro-rs/pull/new/master) with a clear list of what you've done (read more about [pull requests](http://help.github.com/pull-requests/)). When you send a pull request, I will love you forever if you include documentation / a detailed description of what you have done.Please follow the coding conventions (below) and make sure all of your commits are atomic where possible (one feature per commit).

Always write a clear log message for your commits. One-line messages are fine for small changes, but bigger changes should look like this:

    $ git commit -m "A brief summary of the commit
    > 
    > A paragraph describing what changed and its impact."

## Coding conventions

Start reading the code and you should get the hang of it. The codebase is generally aimed for readability:

- Indentation is four spaces (single tab)
- Keep dependency chains small, where possible
- Add documentation where you can using [Rustdoc](https://doc.rust-lang.org/beta/rust-by-example/meta/doc.html)
- Markdown should generally be linted via `markdownlint`, but is not strictly required
- Code should be checked with [Clippy](https://doc.rust-lang.org/clippy/)
- Where possible, depend on rust native features instead of pulling in external dependencies
