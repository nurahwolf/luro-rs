-- Your SQL goes here
CREATE TYPE user_permissions AS ENUM ('USER', 'OWNER', 'ADMINISTRATOR');
CREATE TABLE IF NOT EXISTS users (
    accent_colour INT,
    avatar JSONB,
    avatar_decoration JSONB,
    averagesize BIGINT,
    banner JSONB,
    bot BOOLEAN,
    characters INT[],
    discriminator SMALLINT NOT NULL,
    email TEXT,
    flags JSONB,
    global_name TEXT,
    locale TEXT,
    message_edits BIGINT,
    messages BIGINT[],
    mfa_enabled BOOLEAN,
    moderation_actions JSONB,
    moderation_actions_performed BIGINT,
    name TEXT NOT NULL,
    premium_type JSONB,
    public_flags JSONB,
    system BOOLEAN,
    user_id BIGINT NOT NULL PRIMARY KEY,
    verified BOOLEAN,
    warnings BIGINT[],
    words_average BIGINT,
    words_count BIGINT,

    -- ordsize: Json<BTreeMap<usize, usize>>,
    -- words: Json<BTreeMap<String, usize>>,
    -- warnings: Vec<(String, Id<UserMarker>)>,
    -- marriages: BTreeMap<Id<UserMarker>, UserMarriages>,
    -- guilds: HashMap<Id<GuildMarker>, LuroMember>,
    -- character_prefix: BTreeMap<String, String>,

    user_permissions user_permissions
);