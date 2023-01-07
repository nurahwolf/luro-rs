use rand::{thread_rng, seq::IteratorRandom};

use crate::structs::e621::E621Post;

pub fn random_remove(input: &mut Vec<E621Post>) -> Option<E621Post> {
    let i = (0..input.len()).choose(&mut thread_rng())?;
    Some(input.swap_remove(i))
}