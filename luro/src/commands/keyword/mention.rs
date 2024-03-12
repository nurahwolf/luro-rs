use rand::seq::SliceRandom;

const MENTION_BOT_INSULTS: [&str; 14] = [
    "A fatal error has occured, please report this to- No, just kidding. I just simply don't want to respond to you.",
    "For crimes against skyrim and her people, I have elected to ignore you.",
    "Go away, I don't want to talk to you right now.",
    "Have you considered fucking off?",
    "I have considered your request and decided I simply do not want to respond to you.",
    "I have no interest in talking to you.",
    "I only talk to gays. If you think you are gay, that means you are not gay enough. Sorry.",
    "Lick my paws first and then I might choose to respond to you.",
    "Maybe if you clip this leash around your neck, I might listen to what you have to say.",
    "No.",
    "Pay me first.",
    "Sorry, I'm in boykisser mode. I only speak to boykissers.",
    "That's cute, but no.",
    "Unless you are my bitch and belong to me, I will choose to ignore you.",
];

pub fn choose_insult() -> &'static str {
    MENTION_BOT_INSULTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&MENTION_BOT_INSULTS[0])
}
