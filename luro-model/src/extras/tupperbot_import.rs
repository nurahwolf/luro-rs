use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupperBotImport {
    pub version: i64,
    pub id: String,
    pub uuid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Value,
    pub pronouns: Value,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Value,
    pub banner: Value,
    pub color: Value,
    pub created: String,
    #[serde(rename = "webhook_url")]
    pub webhook_url: Value,
    pub privacy: Privacy,
    pub config: Config,
    pub accounts: Vec<i64>,
    pub members: Vec<TupperBotCharacter>,
    pub groups: Vec<Value>,
    pub switches: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Privacy {
    #[serde(rename = "description_privacy")]
    pub description_privacy: String,
    #[serde(rename = "pronoun_privacy")]
    pub pronoun_privacy: String,
    #[serde(rename = "member_list_privacy")]
    pub member_list_privacy: String,
    #[serde(rename = "group_list_privacy")]
    pub group_list_privacy: String,
    #[serde(rename = "front_privacy")]
    pub front_privacy: String,
    #[serde(rename = "front_history_privacy")]
    pub front_history_privacy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub timezone: String,
    #[serde(rename = "pings_enabled")]
    pub pings_enabled: bool,
    #[serde(rename = "latch_timeout")]
    pub latch_timeout: Value,
    #[serde(rename = "member_default_private")]
    pub member_default_private: bool,
    #[serde(rename = "group_default_private")]
    pub group_default_private: bool,
    #[serde(rename = "show_private_info")]
    pub show_private_info: bool,
    #[serde(rename = "member_limit")]
    pub member_limit: i64,
    #[serde(rename = "group_limit")]
    pub group_limit: i64,
    #[serde(rename = "case_sensitive_proxy_tags")]
    pub case_sensitive_proxy_tags: bool,
    #[serde(rename = "proxy_error_message_enabled")]
    pub proxy_error_message_enabled: bool,
    #[serde(rename = "description_templates")]
    pub description_templates: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TupperBotCharacter {
    pub id: String,   // Done
    pub uuid: String, // Done
    pub name: String, // Done
    #[serde(rename = "display_name")]
    pub display_name: String, // Done
    pub color: Option<String>, // Done
    pub birthday: Value, // Done
    pub pronouns: String, // TODO
    #[serde(rename = "avatar_url")]
    pub avatar_url: String, // Done
    #[serde(rename = "webhook_avatar_url")]
    pub webhook_avatar_url: Value, // Done
    pub banner: Value, // Done
    pub description: String, // Done
    pub created: String, // Done
    #[serde(rename = "keep_proxy")]
    pub keep_proxy: bool, // Done
    #[serde(rename = "autoproxy_enabled")]
    pub autoproxy_enabled: bool, // Done
    #[serde(rename = "message_count")]
    pub message_count: i64, // Done
    #[serde(rename = "last_message_timestamp")]
    pub last_message_timestamp: Option<String>, // Done
    #[serde(rename = "proxy_tags")]
    pub proxy_tags: Vec<ProxyTag>, // Done
    pub privacy: Privacy2, // Done
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyTag {
    pub prefix: String,
    pub suffix: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Privacy2 {
    pub visibility: String,
    #[serde(rename = "name_privacy")]
    pub name_privacy: String,
    #[serde(rename = "description_privacy")]
    pub description_privacy: String,
    #[serde(rename = "birthday_privacy")]
    pub birthday_privacy: String,
    #[serde(rename = "pronoun_privacy")]
    pub pronoun_privacy: String,
    #[serde(rename = "avatar_privacy")]
    pub avatar_privacy: String,
    #[serde(rename = "metadata_privacy")]
    pub metadata_privacy: String,
}
