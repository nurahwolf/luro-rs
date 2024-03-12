use rand::seq::SliceRandom;

const PAW_INSULTS: [&str; 7] = [
    "*Pushes my paws right in front of your face.*",
    "*Starts to get you off with my paws.*",
    "Oh I bet you want to look at my paws, you fucking perv.",
    "Why do all of you degenerates like paws.",
    "I swear to god if you don't shut the fuck up about paws.",
    "If you don't watch it I'm gonna be putting you in chastity just to rub my paws all over you.",
    "I dare you to rub your paws on someone here.",
];

pub fn choose_insult() -> &'static str {
    PAW_INSULTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&PAW_INSULTS[0])
}
