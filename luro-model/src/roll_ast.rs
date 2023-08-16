use core::fmt;
use std::{fmt::Display, num::NonZeroU64};

use crate::{dice_roll::DiceRoll, filter_modifier::FilterModifier, roll_value::RollValue};

const DEFAULT_SIDES: &str = "20";

#[derive(Debug, PartialEq, Clone)]
pub enum RollAst {
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Mod(Box<Self>, Box<Self>),
    IDiv(Box<Self>, Box<Self>),
    Power(Box<Self>, Box<Self>),
    Minus(Box<Self>),
    Dice(Option<Box<Self>>, Option<Box<Self>>, FilterModifier<Box<Self>>, u64),
    Const(String)
}

impl Display for RollAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollAst::Add(l, r) => {
                l.fmt(f)?;
                write!(f, " + ")?;
                r.fmt(f)?;
            }
            RollAst::Sub(l, r) => {
                l.fmt(f)?;
                write!(f, " - ")?;
                r.fmt(f)?;
            }
            RollAst::Mul(l, r) => {
                l.fmt(f)?;
                write!(f, " * ")?;
                r.fmt(f)?;
            }
            RollAst::Div(l, r) => {
                l.fmt(f)?;
                write!(f, " / ")?;
                r.fmt(f)?;
            }
            RollAst::Mod(l, r) => {
                l.fmt(f)?;
                write!(f, " mod ")?;
                r.fmt(f)?;
            }
            RollAst::IDiv(l, r) => {
                l.fmt(f)?;
                write!(f, " // ")?;
                r.fmt(f)?;
            }
            RollAst::Power(l, r) => {
                l.fmt(f)?;
                write!(f, " ** ")?;
                r.fmt(f)?;
            }
            RollAst::Minus(t) => {
                write!(f, "-")?;
                t.fmt(f)?;
            }
            RollAst::Dice(times, sides, fm, _) => {
                if let Some(t) = times {
                    t.fmt(f)?;
                }

                write!(f, "d")?;

                if let Some(s) = sides {
                    s.fmt(f)?;
                }

                fm.fmt(f)?;
            }
            RollAst::Const(s) => f.write_str(s)?
        }

        Ok(())
    }
}

impl RollAst {
    pub fn interp(self, rolls: &mut Vec<(u64, DiceRoll)>) -> Result<RollValue, String> {
        Ok(match self {
            RollAst::Add(l, r) => l.interp(rolls)? + r.interp(rolls)?,
            RollAst::Sub(l, r) => l.interp(rolls)? - r.interp(rolls)?,
            RollAst::Div(l, r) => l.interp(rolls)? / r.interp(rolls)?,
            RollAst::Mul(l, r) => l.interp(rolls)? * r.interp(rolls)?,
            RollAst::Mod(l, r) => l.interp(rolls)? % r.interp(rolls)?,
            RollAst::IDiv(l, r) => (l.interp(rolls)? / r.interp(rolls)?).floor(),
            RollAst::Power(l, r) => l.interp(rolls)?.pow(r.interp(rolls)?),
            RollAst::Minus(l) => -l.interp(rolls)?,
            RollAst::Const(val) => {
                let dots = val.matches('.').count();
                if dots == 0 {
                    RollValue::Int(val.parse::<i64>().map_err(|e| e.to_string())?)
                } else if dots == 1 {
                    RollValue::Float(val.parse::<f64>().map_err(|e| e.to_string())?)
                } else {
                    return Err(format!("{val} couldn't be parsed as number (too many dots)"));
                }
            }

            RollAst::Dice(None, r, fm, dp) => {
                RollAst::Dice(Some(Box::new(RollAst::Const("1".to_string()))), r, fm, dp).interp(rolls)?
            }
            RollAst::Dice(l, None, fm, dp) => {
                RollAst::Dice(l, Some(Box::new(RollAst::Const(DEFAULT_SIDES.to_string()))), fm, dp).interp(rolls)?
            }

            RollAst::Dice(Some(l), Some(r), fm, dp) => {
                if let (RollValue::Int(lv), RollValue::Int(rv)) = (l.interp(rolls)?, r.interp(rolls)?) {
                    let fm_value: FilterModifier<RollValue> = fm.map(|i| i.interp(rolls)).swap()?;

                    let fm_int = fm_value
                        .map(|i| {
                            if let RollValue::Int(v) = i {
                                Ok(v as u64)
                            } else {
                                Err(format!("{i:?}: couldn't be parsed as int"))
                            }
                        })
                        .swap()?;

                    let roll = DiceRoll::roll_die(
                        lv as u64,
                        NonZeroU64::new(rv as u64).ok_or("Can't roll zero sided die")?,
                        fm_int,
                        rand::thread_rng()
                    );
                    let total = roll.total;

                    rolls.push((dp, roll));
                    RollValue::Int(total)
                } else {
                    return Err("couldn't be parsed as dice roll (no ints)".to_string());
                }
            }
        })
    }
}
