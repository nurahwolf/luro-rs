use core::fmt;
use core::option::Option::Some;
use core::result::Result::{Err, Ok};
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::num::NonZeroU64;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Debug, Clone)]
pub struct Roll {
    pub vals: Vec<u64>,
    pub total: i64,
    pub sides: NonZeroU64
}

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

const DIR: &[&str] = &["North", "North East", "East", "South East", "South", "South West", "West", "North West", "Stay"];

pub fn roll_direction() -> String {
    let value = roll_die(1, NonZeroU64::new(DIR.len() as u64).unwrap(), FilterModifier::None, rand::thread_rng());
    DIR[value.total as usize - 1].to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterModifier<T> {
    KeepLowest(T),
    KeepHighest(T),
    DropLowest(T),
    DropHighest(T),
    None
}

impl<T: Display> Display for FilterModifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeepLowest(v) => {
                write!(f, "kl")?;
                v.fmt(f)?
            }
            Self::KeepHighest(v) => {
                write!(f, "kh")?;
                v.fmt(f)?;
            }
            Self::DropLowest(v) => {
                write!(f, "dl")?;
                v.fmt(f)?;
            }
            Self::DropHighest(v) => {
                write!(f, "dh")?;
                v.fmt(f)?;
            }
            Self::None => {}
        }

        Ok(())
    }
}

use std::collections::{HashMap, HashSet};
use std::str::Chars;

use rand::RngCore;

#[derive(Debug, Clone)]
pub struct Options {
    options: HashSet<String>,
    lastpos: u64,
    messages: Vec<String>,
    source: String
}

impl Options {
    pub fn new(source: String) -> Self {
        Self {
            options: HashSet::new(),
            lastpos: 0,
            messages: vec![],
            source
        }
    }

    pub fn message(mut self, msg: impl AsRef<str>) -> Self {
        self.messages.push(msg.as_ref().to_string());
        self
    }

    pub fn pos(mut self, pos: u64) -> Self {
        if pos > self.lastpos {
            self.lastpos = pos;
        }
        self
    }

    pub fn merge(mut self, other: Options) -> Self {
        for i in other.options {
            self = self.add_str(i);
        }

        self
    }

    pub fn add(mut self, value: char) -> Self {
        self.options.insert(value.to_string());
        self
    }

    pub fn add_str(mut self, value: impl AsRef<str>) -> Self {
        self.options.insert(value.as_ref().to_string());
        self
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.source)?;
        writeln!(f, "{}^", " ".repeat(self.lastpos as usize))?;

        if !self.options.is_empty() {
            writeln!(f, "An error occurred: unexpected character.")?;
            write!(f, "Expected any of: [")?;
            for (index, i) in self.options.iter().enumerate() {
                write!(f, "{i}")?;

                if index != self.options.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
            writeln!(f)?;
        }

        for i in &self.messages {
            writeln!(f, "{i}")?;
        }

        Ok(())
    }
}

impl<T> FilterModifier<T> {
    pub(crate) fn map<F, U>(self, f: F) -> FilterModifier<U>
    where
        F: FnOnce(T) -> U
    {
        match self {
            Self::KeepLowest(i) => FilterModifier::KeepLowest(f(i)),
            Self::KeepHighest(i) => FilterModifier::KeepHighest(f(i)),
            Self::DropHighest(i) => FilterModifier::DropHighest(f(i)),
            Self::DropLowest(i) => FilterModifier::DropLowest(f(i)),
            Self::None => FilterModifier::None
        }
    }
}

impl<T, E> FilterModifier<Result<T, E>> {
    pub(crate) fn swap(self) -> Result<FilterModifier<T>, E> {
        Ok(match self {
            FilterModifier::KeepLowest(i) => FilterModifier::KeepLowest(i?),
            FilterModifier::KeepHighest(i) => FilterModifier::KeepHighest(i?),
            FilterModifier::DropLowest(i) => FilterModifier::DropLowest(i?),
            FilterModifier::DropHighest(i) => FilterModifier::DropHighest(i?),
            FilterModifier::None => FilterModifier::None
        })
    }
}

pub const DEFAULT_SIDES: &str = "20";

#[derive(Debug, PartialEq)]
pub enum Value {
    Float(f64),
    Int(i64)
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match v {
            Value::Int(i) => i as f64,
            Value::Float(f) => f
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(v) => f.write_str(&v.to_string()),
            Self::Int(v) => f.write_str(&v.to_string())
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i + j),
            (Value::Int(i), Value::Float(j)) => Value::Float(i as f64 + j),
            (Value::Float(i), Value::Int(j)) => Value::Float(i + j as f64),
            (Value::Int(i), Value::Int(j)) => Value::Int(i + j)
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i - j),
            (Value::Int(i), Value::Float(j)) => Value::Float(i as f64 - j),
            (Value::Float(i), Value::Int(j)) => Value::Float(i - j as f64),
            (Value::Int(i), Value::Int(j)) => Value::Int(i - j)
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i * j),
            (Value::Int(i), Value::Float(j)) => Value::Float(i as f64 * j),
            (Value::Float(i), Value::Int(j)) => Value::Float(i * j as f64),
            (Value::Int(i), Value::Int(j)) => Value::Int(i * j)
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i / j),
            (Value::Int(i), Value::Float(j)) => Value::Float(i as f64 / j),
            (Value::Float(i), Value::Int(j)) => Value::Float(i / j as f64),
            (Value::Int(i), Value::Int(j)) => Value::Float(i as f64 / j as f64)
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i % j),
            (Value::Int(i), Value::Float(j)) => Value::Float(i as f64 % j),
            (Value::Float(i), Value::Int(j)) => Value::Float(i % j as f64),
            (Value::Int(i), Value::Int(j)) => Value::Int(i % j)
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Float(i) => Value::Float(-i),
            Value::Int(i) => Value::Int(-i)
        }
    }
}

impl Value {
    pub fn floor(self) -> Self {
        match self {
            Value::Float(i) => Value::Int(i.floor() as i64),
            i => i
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Float(i), Value::Float(j)) => Value::Float(i.powf(j)),
            (Value::Int(i), Value::Float(j)) => Value::Float((i as f64).powf(j)),
            (Value::Float(i), Value::Int(j)) => Value::Float(i.powf(j as f64)),
            (Value::Int(i), Value::Int(j)) if j < 0 => Value::Float((i as f64).powf(j as f64)),
            (Value::Int(i), Value::Int(j)) => Value::Int(i.pow(j as u32))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Mod(Box<Ast>, Box<Ast>),
    IDiv(Box<Ast>, Box<Ast>),
    Power(Box<Ast>, Box<Ast>),
    Minus(Box<Ast>),
    Dice(Option<Box<Ast>>, Option<Box<Ast>>, FilterModifier<Box<Ast>>, u64),

    Const(String)
}

impl Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Add(l, r) => {
                l.fmt(f)?;
                write!(f, " + ")?;
                r.fmt(f)?;
            }
            Ast::Sub(l, r) => {
                l.fmt(f)?;
                write!(f, " - ")?;
                r.fmt(f)?;
            }
            Ast::Mul(l, r) => {
                l.fmt(f)?;
                write!(f, " * ")?;
                r.fmt(f)?;
            }
            Ast::Div(l, r) => {
                l.fmt(f)?;
                write!(f, " / ")?;
                r.fmt(f)?;
            }
            Ast::Mod(l, r) => {
                l.fmt(f)?;
                write!(f, " mod ")?;
                r.fmt(f)?;
            }
            Ast::IDiv(l, r) => {
                l.fmt(f)?;
                write!(f, " // ")?;
                r.fmt(f)?;
            }
            Ast::Power(l, r) => {
                l.fmt(f)?;
                write!(f, " ** ")?;
                r.fmt(f)?;
            }
            Ast::Minus(t) => {
                write!(f, "-")?;
                t.fmt(f)?;
            }
            Ast::Dice(times, sides, fm, _) => {
                if let Some(t) = times {
                    t.fmt(f)?;
                }

                write!(f, "d")?;

                if let Some(s) = sides {
                    s.fmt(f)?;
                }

                fm.fmt(f)?;
            }
            Ast::Const(s) => f.write_str(s)?
        }

        Ok(())
    }
}

impl Ast {
    pub fn interp(self, rolls: &mut Vec<(u64, Roll)>) -> Result<Value, String> {
        Ok(match self {
            Ast::Add(l, r) => l.interp(rolls)? + r.interp(rolls)?,
            Ast::Sub(l, r) => l.interp(rolls)? - r.interp(rolls)?,
            Ast::Div(l, r) => l.interp(rolls)? / r.interp(rolls)?,
            Ast::Mul(l, r) => l.interp(rolls)? * r.interp(rolls)?,
            Ast::Mod(l, r) => l.interp(rolls)? % r.interp(rolls)?,
            Ast::IDiv(l, r) => (l.interp(rolls)? / r.interp(rolls)?).floor(),
            Ast::Power(l, r) => l.interp(rolls)?.pow(r.interp(rolls)?),
            Ast::Minus(l) => -l.interp(rolls)?,
            Ast::Const(val) => {
                let dots = val.matches('.').count();
                if dots == 0 {
                    Value::Int(val.parse::<i64>().map_err(|e| e.to_string())?)
                } else if dots == 1 {
                    Value::Float(val.parse::<f64>().map_err(|e| e.to_string())?)
                } else {
                    return Err(format!("{val} couldn't be parsed as number (too many dots)"));
                }
            }

            Ast::Dice(None, r, fm, dp) => Ast::Dice(Some(Box::new(Ast::Const("1".to_string()))), r, fm, dp).interp(rolls)?,
            Ast::Dice(l, None, fm, dp) => Ast::Dice(l, Some(Box::new(Ast::Const(DEFAULT_SIDES.to_string()))), fm, dp).interp(rolls)?,

            Ast::Dice(Some(l), Some(r), fm, dp) => {
                if let (Value::Int(lv), Value::Int(rv)) = (l.interp(rolls)?, r.interp(rolls)?) {
                    let fm_value: FilterModifier<Value> = fm.map(|i| i.interp(rolls)).swap()?;

                    let fm_int = fm_value
                        .map(|i| {
                            if let Value::Int(v) = i {
                                Ok(v as u64)
                            } else {
                                Err(format!("{i:?}: couldn't be parsed as int"))
                            }
                        })
                        .swap()?;

                    let roll = roll_die(lv as u64, NonZeroU64::new(rv as u64).ok_or("Can't roll zero sided die")?, fm_int, rand::thread_rng());
                    let total = roll.total;

                    rolls.push((dp, roll));
                    Value::Int(total)
                } else {
                    return Err("couldn't be parsed as dice roll (no ints)".to_string());
                }
            }
        })
    }
}

pub struct RollResult {
    pub string_result: String,
    pub dice_total: Value
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string_result)
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    expr: Peekable<Chars<'a>>,
    pos: u64,
    source: String,

    pub advanced: bool
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            source: expr.to_string(),
            expr: expr.chars().peekable(),
            pos: 0,
            advanced: false
        }
    }

    pub fn backup(&self) -> Self {
        Self {
            expr: self.expr.clone(),
            source: self.source.clone(),
            pos: self.pos,
            advanced: self.advanced
        }
    }

    pub fn restore(&mut self, other: Self) {
        self.expr = other.expr;
        self.pos = other.pos;
        self.source = other.source;
        self.advanced = other.advanced;
    }

    pub fn accept(&mut self, c: char, options: &Options) -> Result<(), Options> {
        self.expect(c, options)?;

        self.pos += 1;
        self.expr.next();
        Ok(())
    }

    pub fn accept_string(&mut self, text: &str, options: &Options) -> Result<(), Options> {
        let backup = self.backup();
        for c in text.chars() {
            if let Err(e) = self.accept(c, options) {
                self.restore(backup);
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn expect(&mut self, c: char, options: &Options) -> Result<(), Options> {
        while let Some(i) = self.expr.peek() {
            if !i.is_whitespace() {
                break;
            }
            self.pos += 1;
            self.expr.next();
        }

        let pk = self.expr.peek();
        if pk == Some(&c) {
            Ok(())
        } else {
            Err(options.clone().add(c).pos(self.pos))
        }
    }

    pub fn accept_any(&mut self, c: &[char], mut options: Options, name: Option<Options>) -> Result<char, Options> {
        for i in c {
            match self.accept(*i, &options) {
                Ok(_) => return Ok(*i),
                Err(o) => {
                    if name.is_none() {
                        options = options.merge(o);
                    }
                }
            }
        }

        if let Some(n) = name {
            options = options.merge(n);
        }

        Err(options.clone())
    }

    pub fn parse(&mut self) -> Result<Ast, Options> {
        let result = self.parse_expr(Options::new(self.source.clone()))?;

        if self.expr.next().is_some() {
            return Err(Options::new(self.source.clone()).pos(self.pos).message("unexpected trailing character(s)"));
        }

        Ok(result)
    }

    pub fn parse_expr(&mut self, options: Options) -> Result<Ast, Options> {
        self.parse_sum(&options)
    }

    pub fn parse_sum(&mut self, options: &Options) -> Result<Ast, Options> {
        let mut res = self.parse_term(options.clone())?;

        while let Ok(op) = self.accept_any(&['+', '-'], options.clone(), None) {
            let right = self.parse_term(options.clone())?;

            res = match op {
                '+' => Ast::Add(Box::new(res), Box::new(right)),
                '-' => Ast::Sub(Box::new(res), Box::new(right)),
                _ => unreachable!()
            }
        }

        Ok(res)
    }

    pub fn parse_term(&mut self, options: Options) -> Result<Ast, Options> {
        let mut res = self.parse_factor(options.clone())?;

        loop {
            let mut options = options.clone();
            let opres = self.accept_any(&['*', '/'], options.clone(), None);
            let mut op = if let Ok(i) = opres {
                i
            } else if self.accept_string("mod", &options).is_ok() {
                '%'
            } else {
                options.add_str("mod").add_str("//");
                break;
            };

            if op == '/' && self.accept('/', &options).is_ok() {
                op = 'i'
            } else {
                options = options.add('/');
            }

            let right = self.parse_factor(options)?;

            res = match op {
                '*' => Ast::Mul(Box::new(res), Box::new(right)),
                '/' => Ast::Div(Box::new(res), Box::new(right)),
                'i' => Ast::IDiv(Box::new(res), Box::new(right)),
                '%' => Ast::Mod(Box::new(res), Box::new(right)),
                _ => unreachable!()
            }
        }

        Ok(res)
    }

    pub fn parse_factor(&mut self, options: Options) -> Result<Ast, Options> {
        let backup = self.backup();

        Ok(match self.accept('-', &options) {
            Ok(_) => Ast::Minus(Box::new(self.parse_power(options)?)),
            Err(o) => {
                self.restore(backup);

                return self.parse_power(o);
            }
        })
    }

    pub fn parse_power(&mut self, options: Options) -> Result<Ast, Options> {
        let mut res = self.parse_atom(options.clone())?;
        if self.accept_string("**", &options).is_ok() {
            let right = self.parse_factor(options)?;
            res = Ast::Power(Box::new(res), Box::new(right));
        }

        Ok(res)
    }

    pub fn parse_atom(&mut self, options: Options) -> Result<Ast, Options> {
        let backup = self.backup();
        Ok(match self.parse_dice(options) {
            Err(mut o) => {
                self.restore(backup);

                let backup = self.backup();
                if self.accept('(', &o).is_ok() {
                    let sm = self.parse_sum(&o)?;
                    self.accept(')', &o).map_err(|e| e.message("missing closing parenthesis"))?;

                    return Ok(sm);
                }
                o = o.add('(').message("tried to parse expression between parenthesis");
                self.restore(backup);

                self.parse_number(&o.message("tried to parse dice roll"))?
            }
            Ok(i) => i
        })
    }

    pub fn parse_dice(&mut self, mut options: Options) -> Result<Ast, Options> {
        let backup = self.backup();

        let rolls = if self.advanced && self.accept('(', &options).is_ok() {
            let sm = self.parse_sum(&options)?;
            self.accept(')', &options).map_err(|e| e.message("missing closing parenthesis"))?;

            Some(Box::new(sm))
        } else {
            if self.advanced {
                options = options.add('(').message("tried to parse expression between parenthesis");
            }
            self.restore(backup);
            self.parse_number(&options).map(Box::new).ok()
        };

        self.accept('d', &options)?;
        let dpos = self.pos - 1;

        let backup = self.backup();
        let sides = if self.advanced && self.accept('(', &options).is_ok() {
            let sm = self.parse_sum(&options)?;
            self.accept(')', &options).map_err(|e| e.message("missing closing parenthesis"))?;

            Some(Box::new(sm))
        } else {
            if self.advanced {
                options = options.add('(').message("tried to parse expression between parenthesis");
            }
            self.restore(backup);

            self.parse_number_or_percent(options.clone()).map(Box::new).ok()
        };

        let fm = if self.accept_string("kh", &options).is_ok() || self.accept('h', &options).is_ok() {
            FilterModifier::KeepHighest(Box::new(self.parse_number(&options).unwrap_or_else(|_| Ast::Const("1".to_string()))))
        } else if self.accept_string("dl", &options).is_ok() || self.accept('l', &options).is_ok() {
            FilterModifier::DropLowest(Box::new(self.parse_number(&options).unwrap_or_else(|_| Ast::Const("1".to_string()))))
        } else if self.accept_string("dh", &options).is_ok() {
            FilterModifier::DropHighest(Box::new(self.parse_number(&options).unwrap_or_else(|_| Ast::Const("1".to_string()))))
        } else if self.accept_string("kl", &options).is_ok() {
            FilterModifier::KeepLowest(Box::new(self.parse_number(&options).unwrap_or_else(|_| Ast::Const("1".to_string()))))
        } else {
            FilterModifier::None
        };

        Ok(Ast::Dice(rolls, sides, fm, dpos))
    }

    pub fn parse_number_or_percent(&mut self, options: Options) -> Result<Ast, Options> {
        if self.accept('%', &options).is_ok() {
            Ok(Ast::Const("100".to_ascii_lowercase()))
        } else {
            self.parse_number(&options.add('%'))
        }
    }

    pub fn parse_number(&mut self, options: &Options) -> Result<Ast, Options> {
        const DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];
        let digits_name = Options::new("".to_string()).add_str("0-9");

        let mut number = vec![self
            .accept_any(DIGITS, options.clone(), Some(digits_name.clone()))
            .map_err(|e| options.clone().merge(e).add('(').message("tried to parse a number"))?];

        loop {
            let backup = self.backup();
            if let Ok(digit) = self.accept_any(DIGITS, options.clone(), Some(digits_name.clone())) {
                number.push(digit);
            } else {
                self.restore(backup);
                break;
            }
        }

        let string: String = number.iter().collect();

        Ok(Ast::Const(string))
    }
}

const STAT_ROLL: &str = "4d6l";
pub fn roll_stats() -> String {
    fn roll_stat() -> Roll {
        let mut rolls = Vec::new();
        Parser::new(STAT_ROLL).parse().unwrap().interp(&mut rolls).unwrap();
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
    let mut p = Parser::new(s);
    p.advanced = advanced;

    let ast = p.parse().map_err(|e| e.to_string())?;

    let copy = ast.clone();

    let mut rolls = Vec::new();
    let total = ast.interp(&mut rolls)?;

    let mut map = HashMap::new();
    for (pos, roll) in rolls {
        map.insert(pos, roll);
    }

    let res = replace_rolls(copy, &map, |roll| format!("{:?}", roll.vals));
    let result: RollResult = RollResult {
        string_result: format!("{s} = {res} = {total}"),
        dice_total: total
    };
    Ok(result)
}

fn replace_rolls(ast: Ast, lookup: &HashMap<u64, Roll>, func: fn(&Roll) -> String) -> Ast {
    return match ast {
        Ast::Add(l, r) => Ast::Add(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Sub(l, r) => Ast::Sub(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Mul(l, r) => Ast::Mul(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Div(l, r) => Ast::Div(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Mod(l, r) => Ast::Mod(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::IDiv(l, r) => Ast::IDiv(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Power(l, r) => Ast::Power(Box::from(replace_rolls(*l, lookup, func)), Box::from(replace_rolls(*r, lookup, func))),
        Ast::Minus(l) => Ast::Minus(Box::from(replace_rolls(*l, lookup, func))),
        Ast::Dice(_, _, _, pos) => {
            // Safety: we exhaustively add all positions to this hashmap so it must contain everything
            // we look up.
            let roll = lookup.get(&pos).unwrap();
            Ast::Const(func(roll))
        }
        x @ Ast::Const(_) => x
    };
}
