use rand::seq::SliceRandom;

const LILY_INSULTS: [&str; 7] = [
    "*Turns on Lamp for Lily.*",
    "*Gently fluffles the good moth friend!",
    "Speaking of Lily... I wonder if she likes paws being pushed against her face?",
    "Lily is such a good girl.",
    "I heard Lily is a pretty good girlkisser. She can also do boykissing too!",
    "Lampfriend!",
    "*Pushes paws against Lily's face.*",
];

pub fn choose_insult() -> &'static str {
    LILY_INSULTS.choose(&mut rand::thread_rng()).unwrap_or(&LILY_INSULTS[0])
}
