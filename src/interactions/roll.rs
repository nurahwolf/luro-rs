use std::sync::RwLockReadGuard;

use anyhow::Result;
use rand::Rng;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{Interaction},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}, gateway::event,
};

use crate::{Luro, event_handler};

#[derive(CommandModel, CreateCommand)]
#[command(name = "roll", desc = "Roll the dice!")]
pub struct Roll { //  - Type 'help' for more infomation TODO
    /// Roll to make e.g 1d20
    roll: String,
}

pub async fn roll_function<'a>(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let data = match Roll::from_interaction(CommandInputData::from(*command_data)) {
        Ok(ok) => ok,
        Err(err) => {
            warn!("Failed to parse interaction data - {err}");
            Roll {roll : String::from(" ")}
        }
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(format!("Sum: {}", calculate(&data.roll.replace(" ", "")))),
            ..Default::default()
        }),
    };

    match luro
        .http
        .interaction(luro.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())

}

/// No Whitespaces 
fn calculate(command: &str) -> i32 {

    let mut value = 0;

    let a_optional = command.split_once("+");
    
    if a_optional.is_some(){
        let (a1, a2) = a_optional.unwrap();
        value += calculate(a1);
        value += calculate(a2);
        return value;
    }

    let s_optional = command.split_once("-");
    
    if s_optional.is_some(){
        let (s1, s2) = s_optional.unwrap();
        value += calculate(s1);
        value -= calculate(s2);
        return value;
    }

    let s_optional = command.split_once("d");
    
    if s_optional.is_some(){
        let (d1, d2) = s_optional.unwrap();

        let d1_value = parse(d1);
        let d2_value = parse(d2);

        if d2_value > 0 && d1_value > 0 {
            let mut gen = rand::thread_rng();
            for _ in 0..d1_value{
                let n = gen.gen_range(0..=d2_value);
                if n == 0 { // work around for xd1. 5d1 = 5
                    value += d2_value;
                    continue;
                }
                value += n;
            }
        }

        return value;
    }else{
        value += parse(command);
    }

    return value;
}

fn parse(number: &str) -> i32{
    return match number.parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,
    };
}