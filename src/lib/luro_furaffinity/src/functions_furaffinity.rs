use crate::structs::furaffinity::FurAffinity;
use crate::{Data, Error, FURAFFINITY_REGEX};

use futures::StreamExt;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{ButtonStyle, CreateComponents, CreateEmbed, CreateMessage, EditMessage, Message};
use poise::serenity_prelude::{Colour, InteractionResponseType};
use poise::{CreateReply, FrameworkContext};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use std::time::Duration;
use std::vec;
