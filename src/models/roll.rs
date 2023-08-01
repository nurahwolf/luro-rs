use std::{collections::HashMap, num::NonZeroU64};

use rand::RngCore;

use crate::models::RollParser;

use super::{FilterModifier, Roll, RollAst, RollResult};

const STAT_ROLL: &str = "4d6l";
const DIR: &[&str] = &[
    "North",
    "North East",
    "East",
    "South East",
    "South",
    "South West",
    "West",
    "North West",
    "Stay"
];

impl Roll {
    pub fn roll_die(times: u64, sides: NonZeroU64, fm: FilterModifier<u64>, mut rng: impl RngCore) -> Roll {
        let mut rolls = Vec::new();
        let range = sides.get();
        for _ in 0..times {
            let roll = (rng.next_u64() % range) + 1;
            rolls.push(roll);
        }

        rolls.sort_unstable();

        match fm {
            FilterModifier::KeepLowest(i) => {
                rolls.truncate(i as usize);
            }
            FilterModifier::KeepHighest(i) => {
                rolls.reverse();
                rolls.truncate(i as usize);
            }
            FilterModifier::DropLowest(i) => {
                rolls.reverse();
                rolls.truncate(rolls.len() - i.min(rolls.len() as u64) as usize);
            }
            FilterModifier::DropHighest(i) => {
                rolls.truncate(rolls.len() - i.min(rolls.len() as u64) as usize);
            }
            FilterModifier::None => {}
        }

        // Shuffle order of results again
        if !rolls.is_empty() {
            let range = rolls.len() as u64;
            for _ in 0..=rolls.len() {
                let a = rng.next_u64() % range + 1;
                let b = rng.next_u64() % range + 1;
                rolls.swap(a as usize - 1, b as usize - 1);
            }
        }

        Roll {
            total: rolls.iter().sum::<u64>() as i64,
            vals: rolls,
            sides
        }
    }

    pub fn roll_direction() -> String {
        let value = Self::roll_die(
            1,
            NonZeroU64::new(DIR.len() as u64).unwrap(),
            FilterModifier::None,
            rand::thread_rng()
        );
        DIR[value.total as usize - 1].to_string()
    }
    pub fn roll_stats() -> String {
        fn roll_stat() -> Roll {
            let mut rolls = Vec::new();
            RollParser::new(STAT_ROLL).parse().unwrap().interp(&mut rolls).unwrap();
            rolls.remove(0).1
        }
        let mut res = String::new();

        for _ in 0..6 {
            let roll = roll_stat();
            res.push_str(&format!("{:2}: {:?}\n", roll.total, roll.vals));
        }
        res
    }

    pub fn roll_inline(s: &str, advanced: bool) -> Result<RollResult, String> {
        let mut p = RollParser::new(s);
        p.advanced = advanced;

        let ast = p.parse().map_err(|e| e.to_string())?;

        let copy = ast.clone();

        let mut rolls = Vec::new();
        let total = ast.interp(&mut rolls)?;

        let mut map = HashMap::new();
        for (pos, roll) in rolls {
            map.insert(pos, roll);
        }

        let res = Self::replace_rolls(copy, &map, |roll| format!("{:?}", roll.vals));
        let result: RollResult = RollResult {
            string_result: format!("{s} = {res} = {total}"),
            dice_total: total
        };
        Ok(result)
    }

    fn replace_rolls(ast: RollAst, lookup: &HashMap<u64, Roll>, func: fn(&Roll) -> String) -> RollAst {
        return match ast {
            RollAst::Add(l, r) => RollAst::Add(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Sub(l, r) => RollAst::Sub(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Mul(l, r) => RollAst::Mul(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Div(l, r) => RollAst::Div(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Mod(l, r) => RollAst::Mod(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::IDiv(l, r) => RollAst::IDiv(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Power(l, r) => RollAst::Power(
                Box::from(Self::replace_rolls(*l, lookup, func)),
                Box::from(Self::replace_rolls(*r, lookup, func))
            ),
            RollAst::Minus(l) => RollAst::Minus(Box::from(Self::replace_rolls(*l, lookup, func))),
            RollAst::Dice(_, _, _, pos) => {
                // Safety: we exhaustively add all positions to this hashmap so it must contain everything
                // we look up.
                let roll = lookup.get(&pos).unwrap();
                RollAst::Const(func(roll))
            }
            x @ RollAst::Const(_) => x
        };
    }
}
