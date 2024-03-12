use rand::seq::SliceRandom;

const LURO_INSULTS: [&str; 12] = [
    "*Cuddles up against her brother.*",
    "*Discreetly starts to grind her body against Luro.*",
    "*Gives a quick kiss to her bro.*",
    "*Quickly kisses Luro on the cheek!*",
    "*Sneakily touches Luro's paws.*",
    "*Sneaks a hand down Luro's pants.*",
    "*Starts to grind herself against Luro's leg, making some rather questionable sounds.*",
    "Aww, you want to speak to me, bro?",
    "Hey Luro!! I missed you!",
    "Luroooooo! I'm soooo horny! Help your sis out! Please!!",
    "Oh brother! Please get me off with your paws!",
    "Oh, hey Luro! How are you today?",
];

pub fn choose_insult() -> &'static str {
    LURO_INSULTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&LURO_INSULTS[0])
}
