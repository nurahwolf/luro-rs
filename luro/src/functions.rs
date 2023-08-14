use std::sync::Arc;

use anyhow::Error;
use luro_model::luro_database_driver::LuroDatabaseDriver;
use luro_model::slash_user::SlashUser;
use twilight_http::client::InteractionClient;
use twilight_model::guild::Member;
use twilight_model::http::interaction::InteractionResponse;
use twilight_model::id::Id;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::user::User;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::framework::Framework;

/// A simple function to respond with `ChannelMessageWithSource`
pub async fn respond_to_interaction(
    interaction_client: &InteractionClient<'_>,
    interaction: &Interaction,
    content: String
) -> Result<(), Error> {
    let data = InteractionResponseDataBuilder::new().content(content).build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data)
    };

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

/// Parse a string into a u32, used for hex codes colours
pub fn parse_string_to_u32(input: String) -> anyhow::Result<u32> {
    Ok(if input.starts_with("0x") {
        u32::from_str_radix(input.as_str().strip_prefix("0x").unwrap(), 16)?
    } else if input.chars().all(|char| char.is_ascii_hexdigit()) {
        u32::from_str_radix(input.as_str(), 16)?
    } else {
        input.parse::<u32>()?
    })
}

/// Work out how many padding characters is needed for a nicely formatted table.
/// This takes a vector containing the word / number lengths in base10, and provices you with the lenth
/// This is broken up by the length of the prefix, suffix and together.
pub fn padding_calculator(input: Vec<(usize, usize)>) -> (usize, usize, usize) {
    let mut prefix_length = 0;
    let mut suffix_length = 0;

    for (prefix, suffix) in input {
        if prefix > prefix_length {
            prefix_length = prefix
        }

        if suffix > suffix_length {
            suffix_length = suffix
        }
    }

    (prefix_length, suffix_length, prefix_length + suffix_length)
}

/// Attempts to fetch the member of the supplied guild_id, otherwise returns the user. This JUST returns the slash_user context.
pub async fn client_fetch<D: LuroDatabaseDriver>(
    ctx: &Arc<Framework<D>>,
    guild_id: Option<Id<GuildMarker>>,
    user_id: Id<UserMarker>
) -> anyhow::Result<SlashUser> {
    match guild_id {
        Some(guild_id) => match client_fetch_member(ctx, guild_id, user_id).await {
            Ok(member) => Ok(member.1),
            Err(_) => Ok(client_fetch_user(ctx, user_id).await?.1)
        },
        None => Ok(client_fetch_user(ctx, user_id).await?.1)
    }
}


    /// Fetch a member using the client. Useful for when you need some additional information
    pub async fn client_fetch_member<D: LuroDatabaseDriver>(
        ctx: &Arc<Framework<D>>,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>
    ) -> anyhow::Result<(Member, SlashUser)> {
        let member = ctx.twilight_client.guild_member(guild_id, user_id).await?.model().await?;

        let mut slash_user = SlashUser {
            user_id,
            user_avatar: member.user.avatar,
            user_banner: member.user.banner,
            user_global_name: member.user.global_name.clone(),
            user_name: member.user.name.clone(),
            user_discriminator: member.user.discriminator,
            member_avatar: member.avatar,
            member_nickname: member.nick.clone(),
            guild_id: Some(guild_id),
            avatar: "".to_owned(),
            name: "".to_owned(),
            banner: None
        };

        slash_user.format();
        Ok((member, slash_user))
    }

    /// Fetch a user using the client. Useful for when you need some additional information
    pub async fn client_fetch_user<D: LuroDatabaseDriver>(
        ctx: &Arc<Framework<D>>,
        user_id: Id<UserMarker>
    ) -> anyhow::Result<(User, SlashUser)> {
        let user = ctx.twilight_client.user(user_id).await?.model().await?;

        let mut slash_user = SlashUser {
            user_id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            user_global_name: user.global_name.clone(),
            member_avatar: None,
            member_nickname: None,
            guild_id: None,
            avatar: "".to_owned(),
            name: "".to_owned(),
            banner: None
        };

        slash_user.format();
        Ok((user, slash_user))
    }