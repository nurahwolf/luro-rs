use anyhow::Context;
use luro_framework::{CommandInteraction, LuroCommand};
use luro_model::types::Gender as LuroGender;
use luro_model::types::Sexuality as LuroSexuality;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "identity", desc = "Set how you identify!")]
pub struct Identity {
    /// What is your gender identity?
    gender: Gender,
    /// What is your sexuality?
    sexuality: Sexuality,
}

#[derive(CommandOption, CreateOption, Debug)]
pub enum Gender {
    #[option(name = "Male", value = "male")]
    Male,
    #[option(name = "Female", value = "female")]
    Female,
    #[option(name = "Female (Trans)", value = "trans_female")]
    TransFemale,
    #[option(name = "Male (Trans)", value = "trans_male")]
    TransMale,
    #[option(name = "It's Complicated", value = "its_complicated")]
    ItsComplicated,
}

#[derive(CommandOption, CreateOption, Debug)]
pub enum Sexuality {
    #[option(name = "Straight", value = "straight")]
    Straight,
    #[option(name = "Bisexual", value = "bisexual")]
    Bisexual,
    #[option(name = "Pansexual", value = "pansexual")]
    Pansexual,
    #[option(name = "Lesbian", value = "lesbian")]
    Lesbian,
    #[option(name = "Gay", value = "gay")]
    Gay,
}

impl LuroCommand for Identity {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut user_data = match ctx.author.data {
            Some(ref data) => data.clone(),
            None => ctx
                .database
                .sqlx
                .get_user_data(ctx.author.user_id)
                .await?
                .context("Expected to get user data")?,
        };

        user_data.gender = Some(match self.gender {
            Gender::Male => LuroGender::Male,
            Gender::Female => LuroGender::Female,
            Gender::TransFemale => LuroGender::TransFemale,
            Gender::TransMale => LuroGender::TransMale,
            Gender::ItsComplicated => LuroGender::ItsComplicated,
        });

        user_data.sexuality = Some(match self.sexuality {
            Sexuality::Straight => LuroSexuality::Straight,
            Sexuality::Bisexual => LuroSexuality::Bisexual,
            Sexuality::Pansexual => LuroSexuality::Pansexual,
            Sexuality::Lesbian => LuroSexuality::Lesbian,
            Sexuality::Gay => LuroSexuality::Gay,
        });

        ctx.database.sqlx.update_user_data(ctx.author.user_id, user_data).await?;

        ctx.respond(|r| r.content("Updated!").ephemeral()).await
    }
}
