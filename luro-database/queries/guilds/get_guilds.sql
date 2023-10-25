SELECT
    guilds.*,
    count(guild_members) as total_members
FROM guilds
JOIN guild_members ON guilds.guild_id = guild_members.guild_id
GROUP BY guilds.guild_id
