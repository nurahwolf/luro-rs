use luro_core::Command;

mod about;
mod boop;
mod cleanup;
mod command_usage;
mod config;
mod context_commands;
mod embed;
mod favs;
mod firstmessage;
mod fursona;
mod guilds;
mod heck;
mod help;
mod image_source;
mod info;
mod invite;
mod lodestonenews;
mod moderator;
mod nickname;
mod owner;
mod ping;
mod printerfacts;
mod quote;
mod say;
mod story;
mod twitter;
mod urban;
mod uwuify;
mod xkcd;

pub fn commands() -> [Command; 31] {
    [
        about::about(),
        boop::boop(),
        cleanup::cleanup(),
        command_usage::command_usage(),
        embed::embed(),
        favs::fav(),
        firstmessage::firstmessage(),
        fursona::fursona(),
        guilds::guilds(),
        heck::heck(),
        heck::heck_user(),
        help::help(),
        image_source::saucenao(),
        info::info(),
        invite::invite(),
        lodestonenews::lodestonenews(),
        nickname::nickname(),
        ping::ping(),
        printerfacts::printerfacts(),
        quote::quote(),
        say::say(),
        story::story(),
        twitter::twitter(),
        urban::random_urban(),
        urban::urban(),
        uwuify::uwu(),
        uwuify::uwuify(),
        xkcd::xkcd(),
        moderator::moderator(),
        owner::owner(),
        context_commands::context_commands()
    ]
}
