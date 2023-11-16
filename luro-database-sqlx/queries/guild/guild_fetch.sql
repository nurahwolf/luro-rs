SELECT
    guilds.*,
    count(guild_members) as total_members,
    array_agg(channels.channel_id) as channels
FROM guilds
LEFT OUTER JOIN guild_members ON guilds.guild_id = guild_members.guild_id
LEFT OUTER JOIN channels ON guilds.guild_id = channels.guild_id
WHERE guilds.guild_id = $1
GROUP BY guilds.guild_id
