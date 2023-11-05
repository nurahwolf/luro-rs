use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, CommandOption, CreateOption};

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
    ItsComplicated
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
    Gay
}


impl LuroCommand for Identity {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let mut user_data = ctx.author.data.clone().unwrap_or(ctx.database.get_user_data(ctx.author.user_id).await?.unwrap_or_default());
        user_data.gender = Some(match self.gender {
            Gender::Male => luro_database::Gender::Male,
            Gender::Female => luro_database::Gender::Female,
            Gender::TransFemale => luro_database::Gender::TransFemale,
            Gender::TransMale => luro_database::Gender::TransMale,
            Gender::ItsComplicated => luro_database::Gender::ItsComplicated,
        });

        user_data.sexuality = Some(match self.sexuality {
            Sexuality::Straight => luro_database::Sexuality::Straight,
            Sexuality::Bisexual => luro_database::Sexuality::Bisexual,
            Sexuality::Pansexual => luro_database::Sexuality::Pansexual,
            Sexuality::Lesbian => luro_database::Sexuality::Lesbian,
            Sexuality::Gay => luro_database::Sexuality::Gay,
        });

        ctx.database.update_user_data(ctx.author.user_id, &user_data).await?;

        ctx.respond(|r|r.content("Updated!").ephemeral()).await
    }
}
