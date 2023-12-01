// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { pub mod public { #[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum UserPermissions { USER,OWNER,ADMINISTRATOR,}impl < 'a > postgres_types :: ToSql for UserPermissions
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, buf : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let s = match * self { UserPermissions :: USER => "USER",UserPermissions :: OWNER => "OWNER",UserPermissions :: ADMINISTRATOR => "ADMINISTRATOR",}
        ; buf.extend_from_slice(s.as_bytes()) ; std :: result :: Result ::
        Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "user_permissions" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 3 { return false ; }
                variants.iter().all(| v | match & * * v
                { "USER" => true,"OWNER" => true,"ADMINISTRATOR" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
} impl < 'a > postgres_types :: FromSql < 'a > for UserPermissions
{
    fn from_sql(ty : & postgres_types :: Type, buf : & 'a [u8],) -> Result <
    UserPermissions, Box < dyn std :: error :: Error + Sync + Send >, >
    {
        match std :: str :: from_utf8(buf) ?
        {
            "USER" => Ok(UserPermissions :: USER),"OWNER" => Ok(UserPermissions :: OWNER),"ADMINISTRATOR" => Ok(UserPermissions :: ADMINISTRATOR),s => Result ::
            Err(Into :: into(format! ("invalid variant `{}`", s))),
        }
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "user_permissions" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 3 { return false ; }
                variants.iter().all(| v | match & * * v
                { "USER" => true,"OWNER" => true,"ADMINISTRATOR" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Gender { MALE,FEMALE,TRANS_FEMALE,TRANS_MALE,ITS_COMPLICATED,}impl < 'a > postgres_types :: ToSql for Gender
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, buf : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let s = match * self { Gender :: MALE => "MALE",Gender :: FEMALE => "FEMALE",Gender :: TRANS_FEMALE => "TRANS_FEMALE",Gender :: TRANS_MALE => "TRANS_MALE",Gender :: ITS_COMPLICATED => "ITS_COMPLICATED",}
        ; buf.extend_from_slice(s.as_bytes()) ; std :: result :: Result ::
        Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "gender" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 5 { return false ; }
                variants.iter().all(| v | match & * * v
                { "MALE" => true,"FEMALE" => true,"TRANS_FEMALE" => true,"TRANS_MALE" => true,"ITS_COMPLICATED" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
} impl < 'a > postgres_types :: FromSql < 'a > for Gender
{
    fn from_sql(ty : & postgres_types :: Type, buf : & 'a [u8],) -> Result <
    Gender, Box < dyn std :: error :: Error + Sync + Send >, >
    {
        match std :: str :: from_utf8(buf) ?
        {
            "MALE" => Ok(Gender :: MALE),"FEMALE" => Ok(Gender :: FEMALE),"TRANS_FEMALE" => Ok(Gender :: TRANS_FEMALE),"TRANS_MALE" => Ok(Gender :: TRANS_MALE),"ITS_COMPLICATED" => Ok(Gender :: ITS_COMPLICATED),s => Result ::
            Err(Into :: into(format! ("invalid variant `{}`", s))),
        }
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "gender" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 5 { return false ; }
                variants.iter().all(| v | match & * * v
                { "MALE" => true,"FEMALE" => true,"TRANS_FEMALE" => true,"TRANS_MALE" => true,"ITS_COMPLICATED" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum Sexuality { STRAIGHT,BISEXUAL,PANSEXUAL,LESBIAN,GAY,}impl < 'a > postgres_types :: ToSql for Sexuality
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, buf : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let s = match * self { Sexuality :: STRAIGHT => "STRAIGHT",Sexuality :: BISEXUAL => "BISEXUAL",Sexuality :: PANSEXUAL => "PANSEXUAL",Sexuality :: LESBIAN => "LESBIAN",Sexuality :: GAY => "GAY",}
        ; buf.extend_from_slice(s.as_bytes()) ; std :: result :: Result ::
        Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "sexuality" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 5 { return false ; }
                variants.iter().all(| v | match & * * v
                { "STRAIGHT" => true,"BISEXUAL" => true,"PANSEXUAL" => true,"LESBIAN" => true,"GAY" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
} impl < 'a > postgres_types :: FromSql < 'a > for Sexuality
{
    fn from_sql(ty : & postgres_types :: Type, buf : & 'a [u8],) -> Result <
    Sexuality, Box < dyn std :: error :: Error + Sync + Send >, >
    {
        match std :: str :: from_utf8(buf) ?
        {
            "STRAIGHT" => Ok(Sexuality :: STRAIGHT),"BISEXUAL" => Ok(Sexuality :: BISEXUAL),"PANSEXUAL" => Ok(Sexuality :: PANSEXUAL),"LESBIAN" => Ok(Sexuality :: LESBIAN),"GAY" => Ok(Sexuality :: GAY),s => Result ::
            Err(Into :: into(format! ("invalid variant `{}`", s))),
        }
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != "sexuality" { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != 5 { return false ; }
                variants.iter().all(| v | match & * * v
                { "STRAIGHT" => true,"BISEXUAL" => true,"PANSEXUAL" => true,"LESBIAN" => true,"GAY" => true,_ => false, })
            } _ => false,
        }
    }
} }}#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod guild_fetch
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq, )] pub struct GuildFetch
{ pub name : String,pub guild_id : i64,pub owner_id : i64,pub afk_timeout : i16,pub verification_level : i16,pub nsfw_level : i16,pub mfa_level : i16,pub explicit_content_filter : i16,pub default_message_notifications : i16,pub system_channel_flags : i64,pub icon : String,pub joined_at : time::OffsetDateTime,pub large : bool,pub max_members : i64,pub system_channel_id : i64,pub safety_alerts_channel_id : i64,pub splash : String,pub afk_channel_id : i64,pub application_id : i64,pub banner : String,pub widget_enabled : bool,pub widget_channel_id : i64,pub owner : bool,pub permissions : i64,pub preferred_locale : String,pub premium_progress_bar_enabled : bool,pub premium_subscription_count : i64,pub premium_tier : i16,pub public_updates_channel_id : i64,pub vanity_url_code : String,pub unavailable : bool,pub rules_channel_id : i64,pub max_video_channel_users : i64,pub max_presences : i64,pub discovery_splash : String,pub approximate_presence_count : i64,pub accent_colour : i32,pub accent_colour_custom : i32,pub moderator_actions_log_channel : i64,pub total_members : i64,pub channels : Vec<i64>,pub role_blacklist : Vec<i64>,}pub struct GuildFetchBorrowed < 'a >
{ pub name : &'a str,pub guild_id : i64,pub owner_id : i64,pub afk_timeout : i16,pub verification_level : i16,pub nsfw_level : i16,pub mfa_level : i16,pub explicit_content_filter : i16,pub default_message_notifications : i16,pub system_channel_flags : i64,pub icon : &'a str,pub joined_at : time::OffsetDateTime,pub large : bool,pub max_members : i64,pub system_channel_id : i64,pub safety_alerts_channel_id : i64,pub splash : &'a str,pub afk_channel_id : i64,pub application_id : i64,pub banner : &'a str,pub widget_enabled : bool,pub widget_channel_id : i64,pub owner : bool,pub permissions : i64,pub preferred_locale : &'a str,pub premium_progress_bar_enabled : bool,pub premium_subscription_count : i64,pub premium_tier : i16,pub public_updates_channel_id : i64,pub vanity_url_code : &'a str,pub unavailable : bool,pub rules_channel_id : i64,pub max_video_channel_users : i64,pub max_presences : i64,pub discovery_splash : &'a str,pub approximate_presence_count : i64,pub accent_colour : i32,pub accent_colour_custom : i32,pub moderator_actions_log_channel : i64,pub total_members : i64,pub channels : cornucopia_async::ArrayIterator<'a, i64>,pub role_blacklist : cornucopia_async::ArrayIterator<'a, i64>,} impl < 'a > From < GuildFetchBorrowed <
'a >> for GuildFetch
{
    fn
    from(GuildFetchBorrowed { name,guild_id,owner_id,afk_timeout,verification_level,nsfw_level,mfa_level,explicit_content_filter,default_message_notifications,system_channel_flags,icon,joined_at,large,max_members,system_channel_id,safety_alerts_channel_id,splash,afk_channel_id,application_id,banner,widget_enabled,widget_channel_id,owner,permissions,preferred_locale,premium_progress_bar_enabled,premium_subscription_count,premium_tier,public_updates_channel_id,vanity_url_code,unavailable,rules_channel_id,max_video_channel_users,max_presences,discovery_splash,approximate_presence_count,accent_colour,accent_colour_custom,moderator_actions_log_channel,total_members,channels,role_blacklist,} : GuildFetchBorrowed < 'a >)
    -> Self { Self { name: name.into(),guild_id,owner_id,afk_timeout,verification_level,nsfw_level,mfa_level,explicit_content_filter,default_message_notifications,system_channel_flags,icon: icon.into(),joined_at,large,max_members,system_channel_id,safety_alerts_channel_id,splash: splash.into(),afk_channel_id,application_id,banner: banner.into(),widget_enabled,widget_channel_id,owner,permissions,preferred_locale: preferred_locale.into(),premium_progress_bar_enabled,premium_subscription_count,premium_tier,public_updates_channel_id,vanity_url_code: vanity_url_code.into(),unavailable,rules_channel_id,max_video_channel_users,max_presences,discovery_splash: discovery_splash.into(),approximate_presence_count,accent_colour,accent_colour_custom,moderator_actions_log_channel,total_members,channels: channels.map(|v| v).collect(),role_blacklist: role_blacklist.map(|v| v).collect(),} }
}pub struct GuildFetchQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GuildFetchBorrowed,
    mapper : fn(GuildFetchBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GuildFetchQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GuildFetchBorrowed) -> R) -> GuildFetchQuery
    < 'a, C, R, N >
    {
        GuildFetchQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn guild_fetch() -> GuildFetchStmt
{ GuildFetchStmt(cornucopia_async :: private :: Stmt :: new("WITH guild AS (
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
LEFT JOIN guild_blacklisted_roles ON guild.guild_id = guild_blacklisted_roles.guild_id")) } pub
struct GuildFetchStmt(cornucopia_async :: private :: Stmt) ; impl
GuildFetchStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
guild_id : & 'a i64,) -> GuildFetchQuery < 'a, C,
GuildFetch, 1 >
{
    GuildFetchQuery
    {
        client, params : [guild_id,], stmt : & mut self.0, extractor :
        | row | { GuildFetchBorrowed { name : row.get(0),guild_id : row.get(1),owner_id : row.get(2),afk_timeout : row.get(3),verification_level : row.get(4),nsfw_level : row.get(5),mfa_level : row.get(6),explicit_content_filter : row.get(7),default_message_notifications : row.get(8),system_channel_flags : row.get(9),icon : row.get(10),joined_at : row.get(11),large : row.get(12),max_members : row.get(13),system_channel_id : row.get(14),safety_alerts_channel_id : row.get(15),splash : row.get(16),afk_channel_id : row.get(17),application_id : row.get(18),banner : row.get(19),widget_enabled : row.get(20),widget_channel_id : row.get(21),owner : row.get(22),permissions : row.get(23),preferred_locale : row.get(24),premium_progress_bar_enabled : row.get(25),premium_subscription_count : row.get(26),premium_tier : row.get(27),public_updates_channel_id : row.get(28),vanity_url_code : row.get(29),unavailable : row.get(30),rules_channel_id : row.get(31),max_video_channel_users : row.get(32),max_presences : row.get(33),discovery_splash : row.get(34),approximate_presence_count : row.get(35),accent_colour : row.get(36),accent_colour_custom : row.get(37),moderator_actions_log_channel : row.get(38),total_members : row.get(39),channels : row.get(40),role_blacklist : row.get(41),} }, mapper : | it | { <GuildFetch>::from(it) },
    }
} }}pub mod marriage_update
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct MarriageUpdateParams < T1 : cornucopia_async::StringSql,> { pub divorced : bool,pub proposer_id : i64,pub proposee_id : i64,pub reason : T1,pub rejected : bool,}pub fn marriage_update() -> MarriageUpdateStmt
{ MarriageUpdateStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO user_marriages (
        divorced,
        proposee_id,
        proposer_id,
        reason,
        rejected
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5
    ) ON CONFLICT (proposer_id, proposee_id) DO
UPDATE
SET divorced = $1,
    proposer_id = $2,
    proposee_id = $3,
    reason = $4,
    rejected = $5")) } pub
struct MarriageUpdateStmt(cornucopia_async :: private :: Stmt) ; impl
MarriageUpdateStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
divorced : & 'a bool,proposer_id : & 'a i64,proposee_id : & 'a i64,reason : & 'a T1,rejected : & 'a bool,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [divorced,proposer_id,proposee_id,reason,rejected,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, MarriageUpdateParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for MarriageUpdateStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    MarriageUpdateParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.divorced,& params.proposer_id,& params.proposee_id,& params.reason,& params.rejected,) ) }
}}pub mod user_fetch
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq, )] pub struct UserFetch
{ pub accent_colour : Option<i32>,pub user_avatar : Option<String>,pub avatar_decoration : Option<String>,pub averagesize : i64,pub user_banner : Option<String>,pub bot : bool,pub characters : Vec<i32>,pub discriminator : i16,pub email : Option<String>,pub user_flags : Option<i64>,pub global_name : Option<String>,pub locale : Option<String>,pub message_edits : i64,pub messages : Vec<i64>,pub mfa_enabled : Option<bool>,pub moderation_actions : serde_json::Value,pub moderation_actions_performed : i64,pub user_name : String,pub premium_type : Option<i16>,pub public_flags : Option<i64>,pub user_system : Option<bool>,pub user_id : i64,pub verified : Option<bool>,pub warnings : Vec<i64>,pub words_average : i64,pub words_count : i64,pub user_permissions : super::super::types::public::UserPermissions,pub gender : Option<super::super::types::public::Gender>,pub sexuality : Option<super::super::types::public::Sexuality>,}pub struct UserFetchBorrowed < 'a >
{ pub accent_colour : Option<i32>,pub user_avatar : Option<&'a str>,pub avatar_decoration : Option<&'a str>,pub averagesize : i64,pub user_banner : Option<&'a str>,pub bot : bool,pub characters : cornucopia_async::ArrayIterator<'a, i32>,pub discriminator : i16,pub email : Option<&'a str>,pub user_flags : Option<i64>,pub global_name : Option<&'a str>,pub locale : Option<&'a str>,pub message_edits : i64,pub messages : cornucopia_async::ArrayIterator<'a, i64>,pub mfa_enabled : Option<bool>,pub moderation_actions : postgres_types::Json<&'a serde_json::value::RawValue>,pub moderation_actions_performed : i64,pub user_name : &'a str,pub premium_type : Option<i16>,pub public_flags : Option<i64>,pub user_system : Option<bool>,pub user_id : i64,pub verified : Option<bool>,pub warnings : cornucopia_async::ArrayIterator<'a, i64>,pub words_average : i64,pub words_count : i64,pub user_permissions : super::super::types::public::UserPermissions,pub gender : Option<super::super::types::public::Gender>,pub sexuality : Option<super::super::types::public::Sexuality>,} impl < 'a > From < UserFetchBorrowed <
'a >> for UserFetch
{
    fn
    from(UserFetchBorrowed { accent_colour,user_avatar,avatar_decoration,averagesize,user_banner,bot,characters,discriminator,email,user_flags,global_name,locale,message_edits,messages,mfa_enabled,moderation_actions,moderation_actions_performed,user_name,premium_type,public_flags,user_system,user_id,verified,warnings,words_average,words_count,user_permissions,gender,sexuality,} : UserFetchBorrowed < 'a >)
    -> Self { Self { accent_colour,user_avatar: user_avatar.map(|v| v.into()),avatar_decoration: avatar_decoration.map(|v| v.into()),averagesize,user_banner: user_banner.map(|v| v.into()),bot,characters: characters.map(|v| v).collect(),discriminator,email: email.map(|v| v.into()),user_flags,global_name: global_name.map(|v| v.into()),locale: locale.map(|v| v.into()),message_edits,messages: messages.map(|v| v).collect(),mfa_enabled,moderation_actions: serde_json::from_str(moderation_actions.0.get()).unwrap(),moderation_actions_performed,user_name: user_name.into(),premium_type,public_flags,user_system,user_id,verified,warnings: warnings.map(|v| v).collect(),words_average,words_count,user_permissions,gender,sexuality,} }
}pub struct UserFetchQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> UserFetchBorrowed,
    mapper : fn(UserFetchBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > UserFetchQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(UserFetchBorrowed) -> R) -> UserFetchQuery
    < 'a, C, R, N >
    {
        UserFetchQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn user_fetch() -> UserFetchStmt
{ UserFetchStmt(cornucopia_async :: private :: Stmt :: new("SELECT *
FROM users
WHERE user_id = $1")) } pub
struct UserFetchStmt(cornucopia_async :: private :: Stmt) ; impl
UserFetchStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
user_id : & 'a i64,) -> UserFetchQuery < 'a, C,
UserFetch, 1 >
{
    UserFetchQuery
    {
        client, params : [user_id,], stmt : & mut self.0, extractor :
        | row | { UserFetchBorrowed { accent_colour : row.get(0),user_avatar : row.get(1),avatar_decoration : row.get(2),averagesize : row.get(3),user_banner : row.get(4),bot : row.get(5),characters : row.get(6),discriminator : row.get(7),email : row.get(8),user_flags : row.get(9),global_name : row.get(10),locale : row.get(11),message_edits : row.get(12),messages : row.get(13),mfa_enabled : row.get(14),moderation_actions : row.get(15),moderation_actions_performed : row.get(16),user_name : row.get(17),premium_type : row.get(18),public_flags : row.get(19),user_system : row.get(20),user_id : row.get(21),verified : row.get(22),warnings : row.get(23),words_average : row.get(24),words_count : row.get(25),user_permissions : row.get(26),gender : row.get(27),sexuality : row.get(28),} }, mapper : | it | { <UserFetch>::from(it) },
    }
} }}pub mod user_update_twilight_user
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UserUpdateTwilightUserParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,> { pub accent_colour : Option<i32>,pub avatar_decoration : Option<T1>,pub bot : bool,pub discriminator : i16,pub email : Option<T2>,pub global_name : Option<T3>,pub locale : Option<T4>,pub mfa_enabled : Option<bool>,pub premium_type : Option<i16>,pub user_avatar : Option<T5>,pub user_banner : Option<T6>,pub user_id : i64,pub user_name : T7,pub user_system : Option<bool>,pub verified : Option<bool>,}pub fn user_update_twilight_user() -> UserUpdateTwilightUserStmt
{ UserUpdateTwilightUserStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO users (
        accent_colour,
        avatar_decoration,
        bot,
        discriminator,
        email,
        global_name,
        locale,
        mfa_enabled,
        premium_type,
        user_avatar,
        user_banner,
        user_id,
        user_name,
        user_system,
        verified
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12,
        $13,
        $14,
        $15
    ) ON CONFLICT (user_id) DO
UPDATE
SET accent_colour = $1,
    avatar_decoration = $2,
    bot = $3,
    discriminator = $4,
    email = $5,
    global_name = $6,
    locale = $7,
    mfa_enabled = $8,
    premium_type = $9,
    user_avatar = $10,
    user_banner = $11,
    user_name = $13,
    user_system = $14,
    verified = $15")) } pub
struct UserUpdateTwilightUserStmt(cornucopia_async :: private :: Stmt) ; impl
UserUpdateTwilightUserStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
accent_colour : & 'a Option<i32>,avatar_decoration : & 'a Option<T1>,bot : & 'a bool,discriminator : & 'a i16,email : & 'a Option<T2>,global_name : & 'a Option<T3>,locale : & 'a Option<T4>,mfa_enabled : & 'a Option<bool>,premium_type : & 'a Option<i16>,user_avatar : & 'a Option<T5>,user_banner : & 'a Option<T6>,user_id : & 'a i64,user_name : & 'a T7,user_system : & 'a Option<bool>,verified : & 'a Option<bool>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [accent_colour,avatar_decoration,bot,discriminator,email,global_name,locale,mfa_enabled,premium_type,user_avatar,user_banner,user_id,user_name,user_system,verified,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UserUpdateTwilightUserParams < T1,T2,T3,T4,T5,T6,T7,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UserUpdateTwilightUserStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UserUpdateTwilightUserParams < T1,T2,T3,T4,T5,T6,T7,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.accent_colour,& params.avatar_decoration,& params.bot,& params.discriminator,& params.email,& params.global_name,& params.locale,& params.mfa_enabled,& params.premium_type,& params.user_avatar,& params.user_banner,& params.user_id,& params.user_name,& params.user_system,& params.verified,) ) }
}}}