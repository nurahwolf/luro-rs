WITH guild_data AS (
    SELECT
        guild_data.*
    FROM guild_data
    WHERE guild_data.guild_id IN (SELECT guild_id FROM guilds)
    GROUP BY guild_data.guild_id
),
guild_members AS (
    SELECT
        guild_members.guild_id,
        count(user_id) as total_members
    FROM guild_members
    WHERE guild_members.guild_id IN (SELECT guild_id FROM guilds)
    GROUP BY guild_members.guild_id
),
guild_channels AS (
    SELECT
        guilds.guild_id,
        array_agg(channels.channel_id) as channels
    FROM guilds
    LEFT JOIN channels ON guilds.guild_id = channels.guild_id
    GROUP BY guilds.guild_id
),
guild_blacklisted_roles AS (
    SELECT
        guilds.guild_id,
        array_agg(guild_role_blacklist.role_id) as role_blacklist
    FROM guilds
    LEFT JOIN guild_role_blacklist ON guilds.guild_id = guild_role_blacklist.guild_id
    GROUP BY guilds.guild_id
)

SELECT
    guilds.*,
    guild_data.accent_colour,
    guild_data.accent_colour_custom,
    guild_data.moderator_actions_log_channel,
    total_members,
    channels,
    role_blacklist
FROM guilds
LEFT JOIN guild_data ON guilds.guild_id = guild_data.guild_id
LEFT JOIN guild_members ON guilds.guild_id = guild_members.guild_id
LEFT JOIN guild_channels ON guilds.guild_id = guild_channels.guild_id
LEFT JOIN guild_blacklisted_roles ON guilds.guild_id = guild_blacklisted_roles.guild_id
