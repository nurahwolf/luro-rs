CREATE VIEW public.guild_member_user AS
SELECT
    NULL::integer AS accent_colour,
    NULL::text AS avatar,
    NULL::text AS avatar_decoration,
    NULL::bigint AS averagesize,
    NULL::text AS banner,
    NULL::boolean AS bot,
    NULL::integer[] AS characters,
    NULL::smallint AS discriminator,
    NULL::text AS email,
    NULL::bigint AS user_flags,
    NULL::text AS global_name,
    NULL::text AS locale,
    NULL::bigint AS message_edits,
    NULL::bigint[] AS messages,
    NULL::boolean AS mfa_enabled,
    NULL::jsonb AS moderation_actions,
    NULL::bigint AS moderation_actions_performed,
    NULL::text AS name,
    NULL::smallint AS premium_type,
    NULL::bigint AS public_flags,
    NULL::boolean AS system,
    NULL::bigint AS user_id,
    NULL::boolean AS verified,
    NULL::bigint[] AS warnings,
    NULL::bigint AS words_average,
    NULL::bigint AS words_count,
    NULL::public.user_permissions AS user_permissions,
    NULL::text AS guild_avatar,
    NULL::timestamp with time zone AS boosting_since,
    NULL::timestamp with time zone AS communication_disabled_until,
    NULL::boolean AS deafened,
    NULL::bigint AS guild_id,
    NULL::timestamp with time zone AS joined_at,
    NULL::bigint AS member_flags,
    NULL::boolean AS muted,
    NULL::text AS nickname,
    NULL::boolean AS pending,
    NULL::bigint[] AS roles;


ALTER VIEW public.guild_member_user OWNER TO nurah;

--
-- Name: guild_members; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.guild_members (
    user_id bigint NOT NULL,
    guild_id bigint NOT NULL,
    member_avatar text,
    boosting_since timestamp with time zone,
    communication_disabled_until timestamp with time zone,
    deafened boolean DEFAULT false NOT NULL,
    member_flags bigint DEFAULT 0 NOT NULL,
    muted boolean DEFAULT false NOT NULL,
    nickname text,
    pending boolean DEFAULT false NOT NULL,
    removed boolean DEFAULT false NOT NULL,
    joined_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    left_at timestamp with time zone,
    guild_owner boolean DEFAULT false NOT NULL
);


ALTER TABLE public.guild_members OWNER TO nurah;

--
-- Name: guild_roles; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.guild_roles (
    role_id bigint NOT NULL,
    guild_id bigint NOT NULL,
    colour integer DEFAULT 0 NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    role_flags bigint DEFAULT 0 NOT NULL,
    hoist boolean DEFAULT false NOT NULL,
    icon text,
    managed boolean DEFAULT false NOT NULL,
    mentionable boolean DEFAULT false NOT NULL,
    role_name text DEFAULT 'UNKNOWN_ROLE'::text NOT NULL,
    permissions bigint DEFAULT 0 NOT NULL,
    "position" bigint DEFAULT 0 NOT NULL,
    tags jsonb,
    unicode_emoji text
);


ALTER TABLE public.guild_roles OWNER TO nurah;

--
-- Name: users; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.users (
    accent_colour integer,
    user_avatar text,
    avatar_decoration text,
    averagesize bigint,
    user_banner text,
    bot boolean DEFAULT false NOT NULL,
    characters integer[],
    discriminator smallint DEFAULT 0 NOT NULL,
    email text,
    user_flags bigint,
    global_name text,
    locale text,
    message_edits bigint,
    messages bigint[],
    mfa_enabled boolean,
    moderation_actions jsonb,
    moderation_actions_performed bigint,
    user_name text DEFAULT 'UNKNOWN_USER'::text NOT NULL,
    premium_type smallint,
    public_flags bigint,
    user_system boolean,
    user_id bigint NOT NULL,
    verified boolean,
    warnings bigint[],
    words_average bigint,
    words_count bigint,
    user_permissions public.user_permissions DEFAULT 'USER'::public.user_permissions NOT NULL,
    gender public.gender,
    sexuality public.sexuality
);


ALTER TABLE public.users OWNER TO nurah;

--
-- Name: TABLE users; Type: COMMENT; Schema: public; Owner: nurah
--

COMMENT ON TABLE public.users IS 'Basic users, additional luro specific data held in other tables';


--
-- Name: guild_user_member_roles; Type: VIEW; Schema: public; Owner: nurah
--

CREATE VIEW public.guild_user_member_roles AS
 SELECT users.accent_colour,
    users.user_avatar AS avatar,
    users.avatar_decoration,
    users.averagesize,
    users.user_banner AS banner,
    users.bot,
    users.characters,
    users.discriminator,
    users.email,
    users.user_flags,
    users.global_name,
    users.locale,
    users.message_edits,
    users.messages,
    users.mfa_enabled,
    users.moderation_actions,
    users.moderation_actions_performed,
    users.user_name AS name,
    users.premium_type,
    users.public_flags,
    users.user_system AS system,
    users.user_id,
    users.verified,
    users.warnings,
    users.words_average,
    users.words_count,
    users.user_permissions,
    guild_members.member_avatar AS guild_avatar,
    guild_members.boosting_since,
    guild_members.communication_disabled_until,
    guild_members.deafened,
    guild_members.guild_id,
    guild_members.joined_at,
    guild_members.member_flags,
    guild_members.muted,
    guild_members.nickname,
    guild_members.pending,
    guild_roles.role_id,
    guild_roles.colour,
    guild_roles.deleted,
    guild_roles.role_flags,
    guild_roles.hoist,
    guild_roles.icon,
    guild_roles.managed,
    guild_roles.mentionable,
    guild_roles.role_name,
    guild_roles.permissions,
    guild_roles."position",
    guild_roles.tags,
    guild_roles.unicode_emoji
   FROM (((public.users
     JOIN public.guild_members ON ((guild_members.user_id = users.user_id)))
     JOIN public.guild_member_roles ON (((guild_member_roles.user_id = users.user_id) AND (guild_members.guild_id = guild_member_roles.guild_id) AND (guild_members.user_id = guild_member_roles.user_id))))
     JOIN public.guild_roles ON ((guild_roles.role_id = guild_member_roles.role_id)));


ALTER VIEW public.guild_user_member_roles OWNER TO nurah;

--
-- Name: guilds; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.guilds (
    name text NOT NULL,
    guild_id bigint NOT NULL,
    owner_id bigint NOT NULL,
    accent_colour integer,
    custom_accent_colour integer,
    afk_timeout smallint NOT NULL,
    verification_level smallint NOT NULL,
    nsfw_level smallint NOT NULL,
    mfa_level smallint NOT NULL,
    explicit_content_filter smallint NOT NULL,
    default_message_notifications smallint NOT NULL,
    system_channel_flags bigint NOT NULL,
    icon text,
    joined_at timestamp with time zone,
    large boolean DEFAULT false NOT NULL,
    max_members bigint,
    system_channel_id bigint,
    safety_alerts_channel_id bigint,
    splash text,
    afk_channel_id bigint,
    application_id bigint,
    banner text,
    widget_enabled boolean,
    widget_channel_id bigint,
    owner boolean,
    permissions bigint,
    preferred_locale text NOT NULL,
    premium_progress_bar_enabled boolean DEFAULT false NOT NULL,
    premium_subscription_count bigint,
    premium_tier smallint,
    public_updates_channel_id bigint,
    vanity_url_code text,
    unavailable boolean DEFAULT false NOT NULL,
    rules_channel_id bigint,
    max_video_channel_users bigint,
    max_presences bigint,
    discovery_splash text,
    approximate_presence_count bigint
);


ALTER TABLE public.guilds OWNER TO nurah;

--
-- Name: images; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.images (
    img_id bigint NOT NULL,
    url text NOT NULL,
    owner_id bigint NOT NULL,
    nsfw boolean NOT NULL,
    source text,
    name text NOT NULL
);


ALTER TABLE public.images OWNER TO nurah;

--
-- Name: images_img_id_seq; Type: SEQUENCE; Schema: public; Owner: nurah
--

ALTER TABLE public.images ALTER COLUMN img_id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.images_img_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: interactions; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.interactions (
    guild_id bigint,
    interaction_id bigint NOT NULL,
    user_id bigint NOT NULL,
    app_permissions bigint,
    application_id bigint NOT NULL,
    channel_id bigint NOT NULL,
    data jsonb,
    guild_locale text,
    kind public.interaction_kind NOT NULL,
    locale text,
    message_id bigint,
    token text NOT NULL,
    member jsonb
);


ALTER TABLE public.interactions OWNER TO nurah;

--
-- Name: materialized_view_name; Type: MATERIALIZED VIEW; Schema: public; Owner: nurah
--

CREATE MATERIALIZED VIEW public.materialized_view_name AS
 SELECT accent_colour,
    user_avatar,
    avatar_decoration,
    averagesize,
    user_banner AS banner,
    bot,
    characters,
    discriminator,
    email,
    user_flags,
    global_name,
    locale,
    message_edits,
    messages,
    mfa_enabled,
    moderation_actions,
    moderation_actions_performed,
    user_name AS name,
    premium_type,
    public_flags,
    user_system AS system,
    user_id,
    verified,
    warnings,
    words_average,
    words_count,
    user_permissions
   FROM public.users
  WITH NO DATA;


ALTER MATERIALIZED VIEW public.materialized_view_name OWNER TO nurah;

--
-- Name: messages; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.messages (
    activity jsonb,
    application jsonb,
    application_id bigint,
    attachments jsonb,
    author jsonb NOT NULL,
    channel_id bigint NOT NULL,
    components jsonb,
    content text,
    deleted boolean,
    edited_timestamp timestamp with time zone,
    embeds jsonb,
    flags jsonb,
    guild_id bigint,
    message_id bigint NOT NULL,
    interaction jsonb,
    kind jsonb NOT NULL,
    member jsonb,
    mention_channels jsonb,
    mention_everyone boolean,
    mention_roles bigint[],
    mentions jsonb,
    message_updates jsonb,
    pinned boolean,
    reactions jsonb,
    reference jsonb,
    referenced_message jsonb,
    role_subscription_data jsonb,
    source public.message_source NOT NULL,
    sticker_items jsonb,
    thread jsonb,
    "timestamp" timestamp with time zone NOT NULL,
    tts boolean,
    webhook_id bigint,
    author_id bigint NOT NULL
);


ALTER TABLE public.messages OWNER TO nurah;

--
-- Name: message_words; Type: VIEW; Schema: public; Owner: nurah
--

CREATE VIEW public.message_words AS
 SELECT activity,
    application,
    application_id,
    attachments,
    author,
    channel_id,
    components,
    content,
    deleted,
    edited_timestamp,
    embeds,
    flags,
    guild_id,
    message_id,
    interaction,
    kind,
    member,
    mention_channels,
    mention_everyone,
    mention_roles,
    mentions,
    message_updates,
    pinned,
    reactions,
    reference,
    referenced_message,
    role_subscription_data,
    source,
    sticker_items,
    thread,
    "timestamp",
    tts,
    webhook_id,
    author_id,
    string_to_array(regexp_replace(content, '[^\w\s]'::text, ''::text, 'g'::text), ' '::text) AS words
   FROM public.messages;


ALTER VIEW public.message_words OWNER TO nurah;

--
-- Name: unique_words; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.unique_words (
    word text NOT NULL,
    count bigint
);


ALTER TABLE public.unique_words OWNER TO nurah;

--
-- Name: user_character_images; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_character_images (
    user_id bigint NOT NULL,
    character_name text NOT NULL,
    favourite boolean NOT NULL,
    img_id bigint NOT NULL
);


ALTER TABLE public.user_character_images OWNER TO nurah;

--
-- Name: user_characters; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_characters (
    user_id bigint NOT NULL,
    character_name text NOT NULL,
    nsfw_description text,
    nsfw_icons text[],
    nsfw_summary text,
    prefix text,
    sfw_description text NOT NULL,
    sfw_icons text[],
    sfw_summary text NOT NULL
);


ALTER TABLE public.user_characters OWNER TO nurah;

--
-- Name: user_characters_fetishes; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_characters_fetishes (
    character_name text NOT NULL,
    fetish_id bigint NOT NULL,
    user_id bigint NOT NULL,
    category public.user_characters_fetishes_category DEFAULT 'NEUTRAL'::public.user_characters_fetishes_category NOT NULL
);


ALTER TABLE public.user_characters_fetishes OWNER TO nurah;

--
-- Name: user_data; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_data (
    user_id bigint NOT NULL
);


ALTER TABLE public.user_data OWNER TO nurah;

--
-- Name: user_marriage_approvals; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_marriage_approvals (
    proposee_id bigint NOT NULL,
    proposer_id bigint NOT NULL,
    user_id bigint NOT NULL,
    approve boolean DEFAULT false NOT NULL,
    disapprove boolean DEFAULT false NOT NULL
);


ALTER TABLE public.user_marriage_approvals OWNER TO nurah;

--
-- Name: user_marriages; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_marriages (
    proposer_id bigint NOT NULL,
    proposee_id bigint NOT NULL,
    divorced boolean DEFAULT true NOT NULL,
    rejected boolean DEFAULT false NOT NULL,
    reason text NOT NULL
);


ALTER TABLE public.user_marriages OWNER TO nurah;

--
-- Name: user_moderation_actions; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_moderation_actions (
    action_id uuid NOT NULL,
    user_id bigint NOT NULL,
    action_taken public.action_taken
);


ALTER TABLE public.user_moderation_actions OWNER TO nurah;

--
-- Name: user_warnings; Type: TABLE; Schema: public; Owner: nurah
--

CREATE TABLE public.user_warnings (
    moderator_id bigint NOT NULL,
    user_id bigint NOT NULL,
    warning text NOT NULL,
    warning_id bigint NOT NULL
);


ALTER TABLE public.user_warnings OWNER TO nurah;

--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: applications applications_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.applications
    ADD CONSTRAINT applications_pkey PRIMARY KEY (application_id);


--
-- Name: channels channels_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.channels
    ADD CONSTRAINT channels_pkey PRIMARY KEY (channel_id);


--
-- Name: fetishes fetishes_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.fetishes
    ADD CONSTRAINT fetishes_pkey PRIMARY KEY (fetish_id);


--
-- Name: guild_member_roles guild_member_roles_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_member_roles
    ADD CONSTRAINT guild_member_roles_pkey PRIMARY KEY (user_id, guild_id, role_id);


--
-- Name: guild_members guild_members_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_members
    ADD CONSTRAINT guild_members_pkey PRIMARY KEY (guild_id, user_id);


--
-- Name: guild_roles guild_roles_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_roles
    ADD CONSTRAINT guild_roles_pkey PRIMARY KEY (role_id, guild_id);


--
-- Name: guilds guilds_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guilds
    ADD CONSTRAINT guilds_pkey PRIMARY KEY (guild_id);


--
-- Name: images images_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.images
    ADD CONSTRAINT images_pkey PRIMARY KEY (img_id);


--
-- Name: interactions interactions_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_pkey PRIMARY KEY (interaction_id);


--
-- Name: messages messages_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.messages
    ADD CONSTRAINT messages_pkey PRIMARY KEY (message_id);


--
-- Name: unique_words unique_words_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.unique_words
    ADD CONSTRAINT unique_words_pkey PRIMARY KEY (word);


--
-- Name: user_character_images user_character_images_pk; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_character_images
    ADD CONSTRAINT user_character_images_pk PRIMARY KEY (user_id, character_name, img_id);


--
-- Name: user_characters_fetishes user_characters_fetishes_pk; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters_fetishes
    ADD CONSTRAINT user_characters_fetishes_pk PRIMARY KEY (user_id, character_name, fetish_id);


--
-- Name: user_characters user_characters_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters
    ADD CONSTRAINT user_characters_pkey PRIMARY KEY (user_id, character_name);


--
-- Name: user_data user_data_pk; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_data
    ADD CONSTRAINT user_data_pk PRIMARY KEY (user_id);


--
-- Name: user_marriage_approvals user_marriage_approvals_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriage_approvals
    ADD CONSTRAINT user_marriage_approvals_pkey PRIMARY KEY (user_id, proposee_id, proposer_id);


--
-- Name: user_marriages user_marriages_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriages
    ADD CONSTRAINT user_marriages_pkey PRIMARY KEY (proposer_id, proposee_id);


--
-- Name: user_moderation_actions user_moderation_actions_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_moderation_actions
    ADD CONSTRAINT user_moderation_actions_pkey PRIMARY KEY (action_id);


--
-- Name: user_moderation_actions user_moderation_actions_user_id_key; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_moderation_actions
    ADD CONSTRAINT user_moderation_actions_user_id_key UNIQUE (user_id);


--
-- Name: user_warnings user_warnings_moderator_id_key; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_warnings
    ADD CONSTRAINT user_warnings_moderator_id_key UNIQUE (moderator_id);


--
-- Name: user_warnings user_warnings_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_warnings
    ADD CONSTRAINT user_warnings_pkey PRIMARY KEY (warning_id);


--
-- Name: user_warnings user_warnings_user_id_key; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_warnings
    ADD CONSTRAINT user_warnings_user_id_key UNIQUE (user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (user_id);


--
-- Name: guild_member_user _RETURN; Type: RULE; Schema: public; Owner: nurah
--

CREATE OR REPLACE VIEW public.guild_member_user AS
 SELECT users.accent_colour,
    users.user_avatar AS avatar,
    users.avatar_decoration,
    users.averagesize,
    users.user_banner AS banner,
    users.bot,
    users.characters,
    users.discriminator,
    users.email,
    users.user_flags,
    users.global_name,
    users.locale,
    users.message_edits,
    users.messages,
    users.mfa_enabled,
    users.moderation_actions,
    users.moderation_actions_performed,
    users.user_name AS name,
    users.premium_type,
    users.public_flags,
    users.user_system AS system,
    users.user_id,
    users.verified,
    users.warnings,
    users.words_average,
    users.words_count,
    users.user_permissions,
    guild_members.member_avatar AS guild_avatar,
    guild_members.boosting_since,
    guild_members.communication_disabled_until,
    guild_members.deafened,
    guild_members.guild_id,
    guild_members.joined_at,
    guild_members.member_flags,
    guild_members.muted,
    guild_members.nickname,
    guild_members.pending,
    array_agg(guild_member_roles.role_id) AS roles
   FROM ((public.users
     JOIN public.guild_members ON ((guild_members.user_id = users.user_id)))
     JOIN public.guild_member_roles ON ((guild_members.user_id = users.user_id)))
  GROUP BY users.user_avatar, users.avatar_decoration, users.user_banner, guild_members.boosting_since, users.bot, users.characters, guild_members.communication_disabled_until, guild_members.deafened, users.discriminator, users.email, users.global_name, guild_members.member_avatar, users.user_id, guild_members.guild_id, guild_members.joined_at, users.locale, guild_members.member_flags, users.message_edits, users.messages, users.mfa_enabled, guild_members.muted, users.user_name, guild_members.nickname, guild_members.pending, users.premium_type, users.public_flags, users.user_system, users.accent_colour, users.user_flags, users.user_permissions, users.verified, users.warnings, users.words_average, users.words_count;


--
-- Name: messages update_unique_words_trigger; Type: TRIGGER; Schema: public; Owner: nurah
--

CREATE TRIGGER update_unique_words_trigger AFTER INSERT ON public.messages FOR EACH ROW EXECUTE FUNCTION public.update_unique_words();


--
-- Name: channels channels_guild_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.channels
    ADD CONSTRAINT channels_guild_id_fkey FOREIGN KEY (guild_id) REFERENCES public.guilds(guild_id);


--
-- Name: fetishes fetishes_creator_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.fetishes
    ADD CONSTRAINT fetishes_creator_fkey FOREIGN KEY (creator) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: guild_member_roles guild_member_roles_guild_members_guild_id_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_member_roles
    ADD CONSTRAINT guild_member_roles_guild_members_guild_id_user_id_fk FOREIGN KEY (guild_id, user_id) REFERENCES public.guild_members(guild_id, user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: guild_member_roles guild_member_roles_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_member_roles
    ADD CONSTRAINT guild_member_roles_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: guild_members guild_members_guild_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_members
    ADD CONSTRAINT guild_members_guild_id_fkey FOREIGN KEY (guild_id) REFERENCES public.guilds(guild_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: guild_members guild_members_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_members
    ADD CONSTRAINT guild_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: guild_roles guild_roles_guild_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guild_roles
    ADD CONSTRAINT guild_roles_guild_id_fkey FOREIGN KEY (guild_id) REFERENCES public.guilds(guild_id);


--
-- Name: guilds guilds_applications_application_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guilds
    ADD CONSTRAINT guilds_applications_application_id_fk FOREIGN KEY (application_id) REFERENCES public.applications(application_id);


--
-- Name: guilds guilds_channels_channel_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guilds
    ADD CONSTRAINT guilds_channels_channel_id_fk FOREIGN KEY (system_channel_id) REFERENCES public.channels(channel_id);


--
-- Name: guilds guilds_channels_channel_id_fk2; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guilds
    ADD CONSTRAINT guilds_channels_channel_id_fk2 FOREIGN KEY (safety_alerts_channel_id) REFERENCES public.channels(channel_id);


--
-- Name: guilds guilds_channels_channel_id_fk3; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.guilds
    ADD CONSTRAINT guilds_channels_channel_id_fk3 FOREIGN KEY (afk_channel_id) REFERENCES public.channels(channel_id);


--
-- Name: images images_users_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.images
    ADD CONSTRAINT images_users_user_id_fk FOREIGN KEY (owner_id) REFERENCES public.users(user_id);


--
-- Name: interactions interactions_application_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_application_id_fkey FOREIGN KEY (application_id) REFERENCES public.applications(application_id);


--
-- Name: interactions interactions_channel_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(channel_id);


--
-- Name: interactions interactions_guild_id_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_guild_id_user_id_fkey FOREIGN KEY (guild_id, user_id) REFERENCES public.guild_members(guild_id, user_id);


--
-- Name: interactions interactions_message_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_message_id_fkey FOREIGN KEY (message_id) REFERENCES public.messages(message_id);


--
-- Name: interactions interactions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.interactions
    ADD CONSTRAINT interactions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- Name: messages messages_users_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.messages
    ADD CONSTRAINT messages_users_user_id_fk FOREIGN KEY (author_id) REFERENCES public.users(user_id);


--
-- Name: user_character_images user_character_images_user_characters_user_id_character_name_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_character_images
    ADD CONSTRAINT user_character_images_user_characters_user_id_character_name_fk FOREIGN KEY (user_id, character_name) REFERENCES public.user_characters(user_id, character_name);


--
-- Name: user_character_images user_character_images_users_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_character_images
    ADD CONSTRAINT user_character_images_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- Name: user_characters_fetishes user_characters_fetishes_fetishes_fetish_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters_fetishes
    ADD CONSTRAINT user_characters_fetishes_fetishes_fetish_id_fk FOREIGN KEY (fetish_id) REFERENCES public.fetishes(fetish_id);


--
-- Name: user_characters_fetishes user_characters_fetishes_user_id_character_name_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters_fetishes
    ADD CONSTRAINT user_characters_fetishes_user_id_character_name_fk FOREIGN KEY (user_id, character_name) REFERENCES public.user_characters(user_id, character_name);


--
-- Name: user_characters_fetishes user_characters_fetishes_users_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters_fetishes
    ADD CONSTRAINT user_characters_fetishes_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- Name: user_characters user_characters_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_characters
    ADD CONSTRAINT user_characters_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_data user_data___fk; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_data
    ADD CONSTRAINT user_data___fk FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_marriage_approvals user_marriage_approvals_proposer_id_proposee_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriage_approvals
    ADD CONSTRAINT user_marriage_approvals_proposer_id_proposee_id_fkey FOREIGN KEY (proposer_id, proposee_id) REFERENCES public.user_marriages(proposer_id, proposee_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_marriage_approvals user_marriage_approvals_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriage_approvals
    ADD CONSTRAINT user_marriage_approvals_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_marriages user_marriages_proposee_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriages
    ADD CONSTRAINT user_marriages_proposee_id_fkey FOREIGN KEY (proposee_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_marriages user_marriages_proposer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_marriages
    ADD CONSTRAINT user_marriages_proposer_id_fkey FOREIGN KEY (proposer_id) REFERENCES public.users(user_id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: user_warnings user_warnings_moderator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_warnings
    ADD CONSTRAINT user_warnings_moderator_id_fkey FOREIGN KEY (moderator_id) REFERENCES public.users(user_id) DEFERRABLE INITIALLY DEFERRED;


--
-- Name: user_warnings user_warnings_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: nurah
--

ALTER TABLE ONLY public.user_warnings
    ADD CONSTRAINT user_warnings_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id) DEFERRABLE INITIALLY DEFERRED;


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: postgres
--

REVOKE USAGE ON SCHEMA public FROM PUBLIC;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--