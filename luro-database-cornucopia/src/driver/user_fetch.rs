use luro_model::types::{User, UserData};
use twilight_model::{
    id::{Id, marker::UserMarker},
    user::{PremiumType, UserFlags},
};

impl crate::Driver {
    pub async fn user_fetch(&self, user_id: Id<UserMarker>) -> anyhow::Result<Option<User>> {
        Ok(crate::cornucopia::queries::user_fetch::user_fetch()
            .bind(&self.client, &(user_id.get() as i64))
            .map(|user| User {
                accent_colour: user.accent_colour.map(|x| x as u32),
                avatar_decoration: crate::handle_img(user.avatar_decoration),
                avatar: crate::handle_img(user.user_avatar),
                banner: crate::handle_img(user.user_banner),
                bot: user.bot,
                data: Some(UserData {
                    user_id: Id::new(user.user_id as u64),
                    permissions: user.user_permissions.into(),
                    gender: user.gender.map(|x| x.into()),
                    sexuality: user.sexuality.map(|x| x.into()),
                }),
                discriminator: user.discriminator as u16,
                email: user.email.map(|x| x.to_owned()),
                flags: user.user_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
                global_name: user.global_name.map(|x| x.to_owned()),
                locale: user.locale.map(|x| x.to_owned()),
                member: None,
                mfa_enabled: user.mfa_enabled,
                name: user.user_name.to_owned(),
                premium_type: user.premium_type.map(|x| PremiumType::from(x as u8)),
                public_flags: user.public_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
                system: user.user_system,
                user_id: Id::new(user.user_id as u64),
                verified: user.verified,
            })
            .opt()
            .await?)
    }
}
