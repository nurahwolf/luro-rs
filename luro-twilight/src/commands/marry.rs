use anyhow::{anyhow, Context};
use luro_framework::{CommandInteraction, ComponentInteraction, CreateLuroCommand, Luro, LuroCommand};

use luro_framework::standard_response::Response;
use luro_model::COLOUR_DANGER;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;
use twilight_model::id::Id;

/// An array of reasons someone would like to marry.
/// TODO: Load this from disk once it's big enough
const MARRIAGE_REASONS: [&str; 68] = [
    "*<author> just opened a box and presented <user> with a shiny tungsten ring! It looks like they want to get closer to each other. Do they accept?*",
    "<author> gives <user> a cheeky grin, a shiny ring hidden behind their back. 'Heard you've been looking for a partner in mischief. How about it?'",
"<author> wraps their fluffy tail around <user>, pulling them close and whispering, 'How about we make it official and be a perfect pair for life?'",
"<author> with a playful grin, 'You know, I heard couples that howl together, stay together. Want to test that theory and marry me?'",
"<author> looks deeply into <user>'s eyes, holding a beautiful sapphire ring. They ask with a hopeful smile, 'Will you marry me?' What's your answer?",
"Under a canopy of stars, <author> playfully bumps their snout against <user>'s, 'What do you say we light up the night together, forever?'",
"<author> shows <user> a photo of a cat and says, 'I can has marriage with you?'",
"<author> hands <user> a squeaky toy shaped like a ring, 'I promise the real one doesn't squeak! How about marrying this silly fur?'",
"With love in their eyes, <author> kneels before <user> and says, 'You complete me, and I want to spend the rest of my life with you. Will you marry me?'",
"In a twist, <author> dons a cat fursuit, holds out a bell collar, and asks <user>, 'Want to be purr-fect together forever?'",
"Whispering into <user>'s ear, <author> teases, 'They say when wolves mate, it's for life. Want to prove that right with me?'",
"Tail wagging excitedly, <author> teases <user>, 'I've sniffed out many paths, but all of them lead to you. Be my lifelong journey?'",
"<author> presents <user> with a ring made out of a twist tie. 'I promise the actual ring will be less... recyclable. Will you marry me?'",
"Looking a bit flustered, <author> says to <user>, 'I've lost my phone number... can I have yours? And while we're at it, your hand in marriage?'",
"<author> cuddles up close to <user>, purring softly, 'You're my sun and stars, my love, will you marry me?'",
"<author> takes <user>'s hands and asks seriously, 'Do you have a map? I keep getting lost... in your eyes. Oh, and will you marry me?'",
"<author> stands before <user>, their heart open and vulnerable, and asks, 'Will you take this journey with me and become my partner for life?'",
"<author> to <user>, 'Are you a rare species? Because I feel so lucky to have found you. Say yes and let's be a wild pair together!'",
"In a dramatic turn, <author> whispers to <user>, 'Much love, very propose. Marry me?'",
"Gazing into <user>'s eyes, <author> playfully says, 'I've got a den, but it feels so empty without you. Want to be my lifelong mate?'",
"In the warm, comfortable den they've shared, <author> turns to <user> and asks, 'Our journey together has been magical, won't you make it eternal?'",
"<author> nuzzles against <user>, their eyes shimmering with mischief, 'Ever thought about being more than just packmates?'",
"<author> looks at <user>, 'I might not have nine lives, but I'd be purr-fectly happy to spend my one life with you. Will you marry me?'",
"<author> hands <user> a ring with a small tag reading 'one size fits all', 'I bought this ring, no returns allowed. So, will you marry me?'",
"<author> presents a ring to <user>, but it's made out of string cheese. They ask, 'Will you brie mine forever?' Thoughts?",
"In a daring move, <author> slides towards <user> with roller skates, but ends up crashing into a pie. They mumble, 'Pie you marry me?'",
"Wagging their tail excitedly, <author> says to <user>, 'Let's be the talk of the pack. Will you marry this wild fur?'",
"<author> presents a ring with a tuft of fur stuck to it, 'Oops, looks like I shed on the ring! But fur-real, will you marry me, <user>?'",
"Hey <user>! <author> has been planning this moment for a long time, and they finally want to ask you to be theirs forever. Will you say yes?",
"<author> builds a giant heart out of redstone blocks and asks <user>, 'Will you be the Diamond to my Pickaxe and join me in our forever adventure?'",
"With a playful growl, <author> presents <user> with a ring and says, 'I promise I won't bite... much. Will you be my forever mate?'",
"<author> brushes their snout against <user>'s and playfully asks, 'Want to howl at the moon with me for the rest of our lives?'",
"<author> pulls out a ring box, opens it and out pops...a chicken nugget? 'Will you be the sauce to my nugget and marry me, <user>?'",
"While trying to serenade <user>, <author> accidentally got tangled in the microphone cord. Do you accept this... *knotty* proposal?",
"<author> whispers softly into <user>'s furry ear, 'Ever thought about how we'd be as life partners? Care to find out?'",
"In a soft, loving voice, <author> whispers to <user>, 'Our bond is the most precious thing to me. Will you make it eternal by marrying me?'",
"<author> circles <user>, admiring them from every angle. 'Every wolf needs a partner. Care to be mine for eternity?'",
"Yikes! <author> rented a hot air balloon to propose, but they're afraid of heights! They yell down to <user>, 'Will you... save me and also marry me?'",
"Under the twinkling stars, <author> presents <user> with a ring, crafted from stardust and dreams. 'In this vast universe, there's only you for me. Will you marry me?'",
"Oops! <author> tried to propose to <user> but dropped the ring into a fish tank! Do you accept this... splashy proposal?",
"<author> gazes at <user> with their bright, sparkling eyes. 'My heart feels at home when I'm with you, will you be my mate for life?'",
"<author> teasingly tugs on <user>'s tail and asks with a sly grin, 'Ready to tie the knot and be mine forever?'",
"<author> nuzzles close to <user>, their heart full of love and anticipation, 'With you, every moment feels like a dream. Will you be my forever mate?'",
"<author> whispers to <user>, 'Do you believe in love at first sight, or should I walk by with this ring again?'",
"<author> presents a collar instead of a ring, 'How about a collar-boration for life? Will you marry me, <user>?'",
"Underneath the starlit sky, <author> turns to <user>, presenting a diamond ring, and asks, 'Will you be the love of my life forever?'",
"<author> nudges <user> playfully, 'I tried chasing my tail, but then I realized I'd rather chase after you. Be my forever mate?'",
"<author> says to <user>, 'I've been reading the book of numbers and realized I don't have yours... or your agreement to marry me. Can I have both?'",
"<author> looks into <user>'s eyes, 'Will you marry me or am I going to have to stalk your Instagram profile forever?'",
"<author> to <user>, 'Are you a magician? Because whenever I look at you, everyone else disappears. Now, let's disappear together into marriage. What do you say?'",
"<author> traces a paw down <user>'s spine, sending a shiver of anticipation. 'Want to be the moon to my howl, forever?'",
"<author> looks at <user> and says, 'If life was a meme, I'd tag you in it every day. Marry me?'",
"Flashing their fangs in a playful grin, <author> says, 'I've marked my territory, and it's you, <user>. Ready to be my lifelong partner?'",
"<author> presents a ring to <user> and asks, 'Do you know da wae... to a happy marriage with me?'",
"With a flick of their tail and a sparkle in their eyes, <author> presents <user> a ring woven from the stars. 'Will you take the leap and join me in this life's journey?'",
"<author> brushes their muzzle against <user>, an intimate sign of affection, and whispers, 'You've captured my heart. Will you be my lifelong partner?'",
"<author> holds up a ball of yarn and a ring to <user>, 'Will you be the playful kitty to my yarn and marry me? Or should I just get another ball of yarn?'",
"Looks like <author> wants to be <user>'s alpha. They're wagging their tail and presenting a collar, asking, 'Will you be my forever mate?'",
"In a grand gesture, <author> tried to use a magic trick to make the ring appear, but now there’s a chicken instead. <user>, will you accept this clucky proposal?",
"<author> tried to use a skywriter plane to propose to <user>, but it ended up saying 'Marry me, Tacos?' instead. Close enough, right?",
"<author> has found the ultimate legendary item: a ring! They present it to <user> and ask, 'Will you be my co-op partner for life?'",
"<author> shares a soft, heartfelt growl to <user>, presenting a radiant, gleaming ring. 'I can't imagine a life without you. Will you be my forever companion?'",
"<author> to <user>, 'Will you marry me? Because I can't seem to imagine a life without you... and your Netflix password.'",
"Looking into <user>'s eyes, <author> takes a deep breath, 'You are the dream I never want to wake up from. Will you be mine forever?'",
"<author> wags their tail furiously, 'I've been trying to sniff out the perfect partner, and I think it's you. Ready for a lifetime of belly rubs and marriage?'",
"<author> dramatically kneels before <user>, 'Will you marry me or should I use this ring to propose to the pizza delivery guy?'",
"Swishing their tail, <author> playfully says, 'My instincts tell me you're purrfect for me. How about making it official and marrying me, <user>?'",
"<author> looks mischievously at <user>, a collar in hand, and teases, 'Ever thought of being collared by me for life?'",
];

mod divorce;
mod marriages;
mod someone;

#[derive(CommandModel, CreateCommand)]
#[command(name = "marry", desc = "Marry a user! Or see who you have married <3")]
pub enum Marry {
    #[command(name = "someone")]
    Someone(someone::Someone),
    #[command(name = "marriages")]
    Marriages(marriages::Marriages),
    #[command(name = "divorce")]
    Divorce(divorce::Divorce),
}

impl CreateLuroCommand for Marry {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Someone(command) => command.interaction_command(ctx).await,
            Self::Marriages(command) => command.interaction_command(ctx).await,
            Self::Divorce(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_component(self, ctx: ComponentInteraction, invoking_interaction: Interaction) -> anyhow::Result<()> {
        let proposer = ctx
            .fetch_user(
                invoking_interaction
                    .member
                    .map(|x| x.user.unwrap().id)
                    .unwrap_or(invoking_interaction.user.unwrap().id),
            )
            .await?;
        let (proposee, divorce_wanted) = match self {
            Self::Someone(command) => (ctx.fetch_user(command.marry.resolved.id).await?, false),
            Self::Divorce(command) => (ctx.fetch_user(command.user.resolved.id).await?, true),
            _ => {
                return ctx
                    .response_simple(Response::InternalError(anyhow!("Can't find the request to marry, sorry!")))
                    .await
            }
        };
        let (proposer_id, proposee_id) = (proposer.user_id, (proposee.user_id));

        let mut embed = ctx.default_embed().await;
        let marriage = ctx
            .database
            .get_marriage(proposer_id, proposee_id)
            .await?
            .context("Expected to find marriage in database")?;

        // Handle if someone else clicked the button
        if ctx.author.user_id != proposee.user_id {
            match &ctx.data.custom_id == "marry-deny" {
                true => {
                    ctx.database
                        .update_marriage_approval(DbUserMarriageApprovals {
                            user_id: ctx.author.user_id.get() as i64,
                            proposer_id: proposer_id.get() as i64,
                            proposee_id: proposee_id.get() as i64,
                            approve: false,
                            disapprove: true,
                        })
                        .await
                }
                false => {
                    ctx.database
                        .update_marriage_approval(DbUserMarriageApprovals {
                            user_id: ctx.author.user_id.get() as i64,
                            proposer_id: proposer_id.get() as i64,
                            proposee_id: proposee_id.get() as i64,
                            approve: true,
                            disapprove: false,
                        })
                        .await
                }
            }?;
        }

        let marriage_approvals = ctx.database.get_marriage_approvals(proposer_id, proposee_id).await?;

        match divorce_wanted {
            false => embed
                .title(format!("{} has proposed!", proposer.name()))
                .thumbnail(|t| t.url(proposer.avatar_url()))
                .create_field("Their Reason", &marriage.reason, false),
            true => embed
                .colour(COLOUR_DANGER)
                .title(format!(
                    "{} has terminated their marriage with {}!",
                    proposer.name(),
                    proposee.name()
                ))
                .thumbnail(|t| t.url(proposer.avatar_url()))
                .create_field("Their Reason", &marriage.reason, false),
        };

        let mut approval_list = vec![];
        let mut disapproval_list = vec![];

        for approver in marriage_approvals {
            if approver.approve {
                approval_list.push(approver.user_id)
            }

            if approver.disapprove {
                disapproval_list.push(approver.user_id)
            }
        }

        match approval_list.is_empty() {
            false => {
                let mut users = String::new();
                for user in &approval_list {
                    match users.is_empty() {
                        true => users.push_str(&format!("<@{user}>")),
                        false => users.push_str(&format!(", <@{user}>")),
                    }
                }
                users.push_str(&format!("\n- **Total:** `{}`", approval_list.len()));
                embed.create_field("Approvers", &users, true)
            }
            true => embed.create_field("Approvers", "None!", true),
        };

        match disapproval_list.is_empty() {
            false => {
                let mut users = String::new();
                for user in &disapproval_list {
                    match users.is_empty() {
                        true => users.push_str(&format!("<@{user}>")),
                        false => users.push_str(&format!(", <@{user}>")),
                    }
                }
                users.push_str(&format!("\n- **Total:** `{}`", disapproval_list.len()));
                embed.create_field("Disapprovers", &users, true)
            }
            true => embed.create_field("Disapprovers", "None!", true),
        };

        // Handle if someone else clicked the button
        if ctx.author.user_id != proposee.user_id {
            return ctx.respond(|r| r.update().add_embed(embed)).await;
        }

        // Handle if the marriage was denied
        if &ctx.data.custom_id == "marry-deny" {
            if divorce_wanted {
                embed
                .title(format!("Divorce - The worst outcome for poor {}", proposee.name()))
                .description("They cannot believe their fate and have voted to say this action is unfair. Hopefully someone will be able to mend them soon...");
            }

            ctx.database
                .update_marriage(DbUserMarriage {
                    proposer_id: proposer_id.get() as i64,
                    proposee_id: proposee_id.get() as i64,
                    divorced: true,
                    rejected: true,
                    reason: marriage.reason,
                })
                .await?;

            return ctx.respond(|r| r.update().add_embed(embed).components(|c| c)).await;
        }

        let content;
        match divorce_wanted {
            true => {
                content = "".to_owned();
                embed
                .title(format!("Divorce - An acceptable outcome for {}", proposee.name()))
                .description(format!("They are happy with this outcome. Maybe it was just. Maybe it was time to move on. Who knows, maybe **you** will be {}'s next true love.", proposee.name()));
            }
            false => {
                content = format!("Congratulations <@{}> & <@{}>!!!", &proposer_id, &proposee_id);
                embed.title("The marriage proceeded! May they live happily forever after!");
            }
        }

        ctx.database
            .update_marriage(DbUserMarriage {
                proposer_id: proposer_id.get() as i64,
                proposee_id: proposee_id.get() as i64,
                divorced: divorce_wanted,
                rejected: false,
                reason: marriage.reason,
            })
            .await?;

        ctx.respond(|r| r.content(content).update().add_embed(embed).components(|c| c))
            .await
    }
}

/// create components
fn buttons() -> Vec<Component> {
    vec![Component::ActionRow(ActionRow {
        components: vec![
            Component::Button(Button {
                custom_id: Some("marry-accept".to_owned()),
                disabled: false,
                emoji: None,
                label: Some("Do you accept?".to_owned()),
                style: ButtonStyle::Primary,
                url: None,
            }),
            Component::Button(Button {
                custom_id: Some("marry-deny".to_owned()),
                disabled: false,
                emoji: None,
                label: Some("Do you deny?".to_owned()),
                style: ButtonStyle::Danger,
                url: None,
            }),
        ],
    })]
}
