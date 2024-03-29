use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "muzzle", desc = "Put a muzzle on a user")]
pub struct Command {
    /// The user to muzzle.
    user: Id<UserMarker>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        // TODO: Load these from a text file
        let responses = [
            "<user> just got muzzled for a few seconds!!",
            "<user> just got slapped on the muzzle and told to hush.",
            "<user> just got spanked and told to hush up immediately!",
            "<user> was forced on their knees and told to beg to be allowed to speak again.",
            "<user> just had duct tape wrapped around their mouth!",
            "A ballgag was stuffed into <user>'s mouth!",
            "<user> was very naughty.",
            "<user> deserves punishment for speaking when they should not.",
            "<user> was knotted on both ends in order to get them to shut up.",
            "A flirtatious wink from across the room left <user> stumbling over their words.",
            "<author> gave <user> a pinch on the cheek, silencing their words.",
            "<author> gave <user> a swat on the tail, leaving them too surprised to speak.",
            "Oops, someone just hit the mute button on <user> during their karaoke solo.",
            "A soft plushie tail was wrapped around <user>'s muzzle, leaving them blushing and silent.",
            "<author> released a swarm of butterflies around <user>. Their chatter got lost amidst the excited chases.",
            "<author> tossed a squeaky toy to <user>, replacing their words with amusing squeaks.",
            "A playful sprite zipped in, stealing <user>'s voice for a brief moment.",
            "<author> playfully wagged their tail in <user>'s face, causing a flurry of fur that muffled their words.",
            "<user> was given a challenge to catch their own tail. Now they're too busy spinning to speak.",
            "Someone tossed a fish at <user> just to keep them quiet.",
            "<author> swiped a feather over <user>'s muzzle, tickling them into sneezing instead of talking.",
            "Looks like <user> has been lured away from their speech by the mysterious scent of a fox's scent.",
            "For a bit too much noise, <user>'s beak was cloaked with a feather!",
            "<author> used their tail to tickle <user>'s nose, silencing them with giggles.",
            "<user> was ambushed by a playful litter of kittens, muffling their words with purrs and meows.",
            "<author> surprised <user> with a firm paw pat, causing them to pause mid-sentence.",
            "A daring bunny hopped onto <user>'s lap, their surprise silencing their words.",
            "<user> was buried in a pile of yarn balls so they couldn't talk.",
            "<author> flashed a charming smile at <user>, making them forget what they were saying.",
            "<author> gave <user> a mischievous smirk, causing them to lose their train of thought.",
            "<author> started a playful chase with <user>, who ran off mid-sentence.",
            "<author> made <user> laugh with a funny face, interrupting their speech.",
            "<user> was about to speak when a troupe of monkeys started a loud and raucous game of tag.",
            "<author> flashed a flirtatious smile at <user>, causing them to stammer and forget what they were saying.",
            "<author> gave <user> a wink that left them blushing and tongue-tied.",
            "A group of mischievous squirrels just orchestrated a nutty intervention for <user>.",
            "<user> just got a playful paw patting on their muzzle, causing them to giggle instead of speak.",
            "<author> challenged <user> to a game of predator and prey, leaving them too breathless to speak.",
            "Just like that, a colorful parrot swooped in and squawked over <user>.",
            "<author> playfully ran a paw down <user>'s spine, causing them to shiver and lose their words.",
            "A playful nudge on the shoulder made <user> lose their train of thought.",
            "An unplanned pillow fight has caused <user> to lose their breath, and their words.",
            "Oh dear, looks like <user> accidentally sipped some very potent truth serum!",
            "Out of nowhere, a spicy chili pepper has left <user> speechless and gasping for water!",
            "A pack of coyotes decided to sing the moon a serenade, overpowering <user>'s voice.",
            "<author> let out a mighty roar that left <user> speechless.",
            "<author> whispered a forest secret in <user>'s ear, leaving them stunned and silent.",
            "<author> traced a paw along <user>'s whiskers, causing them to purr and lose their words.",
            "<user> was given a honey treat, now they can't stop licking long enough to speak.",
            "<author> placed a playful paw over <user>'s mouth, silencing them.",
            "<author> flashed <user> a wink, leaving them speechless.",
            "<user> was playfully tackled into a ball pit for talking too much.",
            "Oh no, <user> got their head stuck in a beehive, muffling their words!",
            "<author> surprised <user> with a playful wolf's howl, causing them to blush and forget their words.",
            "<author> shared a secret forest melody with <user>, leaving them too mesmerized to talk.",
            "A playful otter just invited <user> to a shell-cracking contest, silencing their talk.",
            "<author> playfully howled a moon-song, leaving <user> awestruck and silent.",
            "<user>'s words were drowned out by the mysterious howling of a distant wolf pack.",
            "<author> started a howling contest with <user>. Their words are now only understandable by the moon.",
            "<author> whispered a naughty limerick into <user>'s ear, causing them to laugh and lose their train of thought.",
            "<user> just got a feather duster ran over their muzzle, tickling them into silence!",
            "<author> swept their tail over <user>'s mouth, muffling them.",
            "<user> was about to say something when a clever crow flew off with their words!",
            "<user>'s words got buried in a pile of fluffy pillows.",
            "<author> challenged <user> to balance a fish on their nose. Now they're too busy concentrating to speak!",
            "<author> tossed a frisky fox at <user>, who was too busy fending it off to speak.",
            "<author> bit <user>'s ear, causing them to yelp and lose their train of thought.",
            "A mysterious paw just swatted <user>'s muzzle shut.",
            "<author> surprised <user> with a quick game of 'Catch the Tail', leaving them panting and speechless.",
            "An impish weasel just swept <user>'s next words right out of their mouth.",
            "<user> was caught by surprise as a velvet ribbon was gently tied around their muzzle.",
            "<author> just initiated a play-fight with <user>, leaving them no time to talk.",
            "<author> caught <user> off guard with a gentle tail caress, rendering them tongue-tied.",
            "<author> tricked <user> into chasing their own tail, leaving them too dizzy to talk.",
            "<author> presented <user> with a rubber duck, quacking so loudly that all chatter was lost.",
            "Just as <user> was about to speak, they got startled by their own echo.",
            "Out of nowhere, a magic 8-ball appeared and answered for <user>, no more words needed.",
            "A mischievous sprite turned <user>'s words into bubbles, popping before they could be heard.",
            "<author> just held up a mirror to <user>, their own fur-style left them speechless!",
            "<user> just got a whisker-tickling butterfly landing on their nose, distracting them from their speech!",
            "<author> wrapped <user> in a warm, fluffy tail hug, making them forget what they were saying.",
            "<author> showed <user> a dance of the fireflies, leaving them too enchanted to speak.",
            "<user> just got their tongue tied up in a knot, thanks to a feisty piece of spaghetti.",
            "<user>'s lips got entangled in a playful game of tug of war with a stuffed animal.",
            "<user> was bundled into a cozy fur pile, muffling their words.",
            "<author> caught <user> off guard with a tickle, causing them to giggle instead of speak.",
            "<author> sneakily covered <user>'s mouth with a fluffy tail, causing a pause in their chatter.",
            "<author> caught <user> off-guard with a playful nuzzle, stopping their chatter.",
            "<author> just pointed out a squirrel to <user>. They're too busy excitedly chasing the squirrel to speak!",
            "<author> blew a bunch of dandelion seeds at <user>, leaving them too busy sneezing to speak.",
            "<author> tried to teach <user> to do the fox-trot. The result was more tangled paws than words.",
            "A funny bird decided to nest on <user>'s head, making it hard for them to continue speaking.",
            "A friendly ghost just possessed <user> and they can only speak in riddles now.",
            "<user> has been given a noisy squeaky toy, now all we hear is squeaks.",
            "<author> sent a fluffy bunny hopping onto <user>'s lap, distracting them from their speech.",
            "<user>'s words were halted when a flock of colorful birds started an impromptu dance around them.",
            "A lovable hedgehog rolled onto <user>'s lap, their surprise silencing them.",
            "<user>'s words were swallowed up by the sudden thunderous stomping of a herd of bison.",
            "<author> playfully patted <user> on the head, quieting their chatter.",
            "A mischievous raccoon just stole <user>'s voice and hid it up a tree!",
            "<user>'s mouth was gently pawed closed by an invisible friend.",
            "<user>'s chatter was interrupted by a cuddly bear demanding a belly rub.",
            "<user>'s words just got lost in the sudden disco lights and funk music!",
            "<author> landed a spank on <user>, leaving them too flustered to talk.",
            "Oh dear, looks like <user> got their muzzle caught in a tub of mint icecream!",
            "<user> just got their muzzle gently covered with a fluffy paw, causing a sudden silence.",
            "<author> surprised <user> with a feather-light kiss on the cheek, causing a blush and a loss for words.",
            "<author> gave <user> a teasing lick on the nose, causing a blush and silence.",
            "<user> had to pause their conversation due to a pesky furball stuck on their tongue.",
            "<author> brushed a feather against <user>'s ears, the tickling sensation leaving them speechless.",
            "<author> distracted <user> with a playful nuzzle, quieting them.",
            "<author> pulled a playful prank on <user> with a fake mouse, scaring the words right out of them!",
            "Someone threw a chew toy at <user> just to keep their chatter at bay.",
            "Oh no, <user> tripped over their own words and landed in a pile of feathers!",
            "<user> just got their fur mysteriously ruffled, leaving them blushing and speechless.",
            "<author> sent a rogue squirrel to tug <user>'s tail, leaving them yelping instead of talking.",
            "It seems a fluffy cloud decided to rain down cotton balls on <user>, halting their words.",
            "<author> offered <user> a honey-sweet nuzzle, their shared giggle cutting off any chatter.",
            "Someone just let out a beastly roar, silencing <user> mid-sentence.",
            "<user>'s monologue got interrupted by the sudden urge to chase their own tail!",
            "A marshmallow was unexpectedly shoved into <user>'s mouth!",
            "<author> gave <user> a sly smile, causing them to lose their words.",
            "<user> was caught off-guard by a mischievous tail tickling their nose, halting their words.",
            "<author> tossed <user> a spicy 'dragon's pepper', leaving them too busy gasping for breath to speak.",
            "<author> tackled <user>, leaving them too winded to speak.",
            "<user> had their paw held unexpectedly, distracting them from their chatter.",
            "<author> just playfully tossed a ball of yarn at <user>, tying their words into knots.",
            "<author> started a dance-off with <user>. They're too busy busting moves to bust out words.",
            "<author> presented <user> with a playful mock fight challenge, leaving them too busy panting to speak.",
            "<user> just got a playful peck on the cheek, causing them to forget their words.",
            "<author> launched a surprise tickle attack on <user>, their words replaced with laughter.",
            "Suddenly, a flock of parrots swooped in, repeating <user>'s last words in a cacophony, drowning out their further chatter.",
            "Oh dear, it seems a family of rabbits decided to use <user> as their playground, interrupting their monologue.",
            "Someone flirtatiously flicked <user>'s ear, causing their words to stutter.",
            "<user> just got their whiskers ruffled, leaving them speechless.",
            "A playful paw just swiped over <user>'s muzzle, causing a pause in their chatter.",
            "A playful dolphin splashed a wave over <user>, silencing their words with laughter.",
            "<author> just showed <user> how to preen their fur, leaving them speechless with the result.",
            "A plush toy was playfully thrown at <user>, leaving them flustered and quiet.",
            "<author> snuck up behind <user> and roared, causing them to jump and lose their words.",
            "<author> challenged <user> to a game of fetch, their words were lost in the excitement.",
            "<user> just got their tail stepped on to make them zip their lip!",
            "<author> playfully waggled their eyebrows at <user>, leaving them flustered and without words.",
            "<user> was caught off guard when a flirty fox stole their next line.",
            "<author> teasingly covered <user>'s mouth with a soft paw.",
            "<user> just had their chatter drowned out by the inexplicably loud romance novel audiobook playing in the background.",
            "A sassy squirrel just tossed an acorn into <user>'s mouth, making their words sound all nutty.",
            "Whoops! A frisky ferret ran off with <user>'s next sentence.",
            "<author> gently tucked a feather in <user>'s muzzle, silencing them.",
            "A playful purr in <user>'s ear sent shivers down their spine, silencing them for a moment.",
            "<user> just got their tongue tied in knots by a cheeky piece of licorice candy.",
            "<author> just playfully muzzled <user> with a paw.",
            "<author> gave <user> a flick on the nose, making them lose their words.",
            "<author> delivered a quick nip to <user>'s tail, silencing them.",
            "<user> has been given a drum, they are only allowed to communicate through beats now.",
            "The room was filled with the sudden uproarious laughter of hyenas, drowning out <user>'s voice.",
            "A raccoon has found a shiny object in <user>'s pocket, distracting them from their chatter.",
            "<author>'s tail found its way to <user>'s lips, causing a pause.",
            "<user>'s words were silenced by the sudden, booming voice of the narrator.",
            "A sly fox just swiped <user>'s words away with a swish of its tail.",
        ];

        ctx.respond(|r| {
            r.content(
                fastrand::choice(responses)
                    .expect("The array is hardcoded, this should not error.")
                    .replace("<user>", format!("<@{}>", self.user).as_str())
                    .replace("<author>", format!("<@{}>", ctx.author_id()).as_str()),
            )
        })
        .await
    }
}
