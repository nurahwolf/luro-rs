use std::{num::NonZeroU64, collections::HashSet, iter::Peekable, str::Chars};

pub mod dice_roll;
pub mod filter_modifier;
pub mod roll_ast;
pub mod roll_options;
pub mod roll_parser;
pub mod roll_result;
pub mod roll_value;

#[derive(Debug, Clone)]
pub struct DiceRoll {
    pub vals: Vec<u64>,
    pub total: i64,
    pub sides: NonZeroU64
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterModifier<T> {
    KeepLowest(T),
    KeepHighest(T),
    DropLowest(T),
    DropHighest(T),
    None
}


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

#[derive(Debug, Clone)]
pub struct RollOptions {
    options: HashSet<String>,
    lastpos: u64,
    messages: Vec<String>,
    source: String
}

#[derive(Debug)]
pub struct RollParser<'a> {
    expr: Peekable<Chars<'a>>,
    pos: u64,
    source: String,

    pub advanced: bool
}

pub struct RollResult {
    pub string_result: String,
    pub dice_total: RollValue
}

#[derive(Debug, PartialEq)]
pub enum RollValue {
    Float(f64),
    Int(i64)
}