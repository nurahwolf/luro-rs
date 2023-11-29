WITH guild AS (
    SELECT
        guilds.*
    FROM guilds
    WHERE guild_id = $1
),
guild_data AS (
    SELECT
        guild_data.*
    FROM guild_data
    WHERE guild_id = $1
),
guild_members AS (
    SELECT
        guild_id,
        count(user_id) as total_members
    FROM guild_members
    WHERE guild_id = $1
    GROUP BY guild_id
),
guild_channels AS (
    SELECT
        guild_id,
        array_agg(channels.channel_id) as channels
    FROM channels
    WHERE guild_id = $1
    GROUP BY guild_id
),
guild_blacklisted_roles AS (
    SELECT
        guild_id,
        array_agg(guild_role_blacklist.role_id) as role_blacklist
    FROM guild_role_blacklist
    WHERE guild_id = $1
    GROUP BY guild_id
)

SELECT
    guild.*,
    guild_data.accent_colour,
    guild_data.accent_colour_custom,
    guild_data.moderator_actions_log_channel,
    total_members,
    channels,
    role_blacklist
FROM guild
LEFT JOIN guild_data ON guild.guild_id = guild_data.guild_id
LEFT JOIN guild_members ON guild.guild_id = guild_members.guild_id
LEFT JOIN guild_channels ON guild.guild_id = guild_channels.guild_id
LEFT JOIN guild_blacklisted_roles ON guild.guild_id = guild_blacklisted_roles.guild_id
