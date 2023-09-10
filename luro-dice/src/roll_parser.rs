use crate::{FilterModifier, RollAst, RollOptions, RollParser};

impl<'a,> RollParser<'a,> {
    pub fn new(expr: &'a str,) -> Self {
        Self {
            source: expr.to_string(),
            expr: expr.chars().peekable(),
            pos: 0,
            advanced: false,
        }
    }

    pub fn backup(&self,) -> Self {
        Self {
            expr: self.expr.clone(),
            source: self.source.clone(),
            pos: self.pos,
            advanced: self.advanced,
        }
    }

    pub fn restore(&mut self, other: Self,) {
        self.expr = other.expr;
        self.pos = other.pos;
        self.source = other.source;
        self.advanced = other.advanced;
    }

    pub fn accept(&mut self, c: char, options: &RollOptions,) -> Result<(), RollOptions,> {
        self.expect(c, options,)?;

        self.pos += 1;
        self.expr.next();
        Ok((),)
    }

    pub fn accept_string(&mut self, text: &str, options: &RollOptions,) -> Result<(), RollOptions,> {
        let backup = self.backup();
        for c in text.chars() {
            if let Err(e,) = self.accept(c, options,) {
                self.restore(backup,);
                return Err(e,);
            }
        }

        Ok((),)
    }

    pub fn expect(&mut self, c: char, options: &RollOptions,) -> Result<(), RollOptions,> {
        while let Some(i,) = self.expr.peek() {
            if !i.is_whitespace() {
                break;
            }
            self.pos += 1;
            self.expr.next();
        }

        let pk = self.expr.peek();
        if pk == Some(&c,) {
            Ok((),)
        } else {
            Err(options.clone().add_value(c,).pos(self.pos,),)
        }
    }

    pub fn accept_any(
        &mut self,
        c: &[char],
        mut options: RollOptions,
        name: Option<RollOptions,>,
    ) -> Result<char, RollOptions,> {
        for i in c {
            match self.accept(*i, &options,) {
                Ok(_,) => return Ok(*i,),
                Err(o,) => {
                    if name.is_none() {
                        options = options.merge(o,);
                    }
                }
            }
        }

        if let Some(n,) = name {
            options = options.merge(n,);
        }

        Err(options.clone(),)
    }

    pub fn parse(&mut self,) -> Result<RollAst, RollOptions,> {
        let result = self.parse_expr(RollOptions::new(self.source.clone(),),)?;

        if self.expr.next().is_some() {
            return Err(RollOptions::new(self.source.clone(),)
                .pos(self.pos,)
                .message("unexpected trailing character(s)",),);
        }

        Ok(result,)
    }

    pub fn parse_expr(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        self.parse_sum(&options,)
    }

    pub fn parse_sum(&mut self, options: &RollOptions,) -> Result<RollAst, RollOptions,> {
        let mut res = self.parse_term(options.clone(),)?;

        while let Ok(op,) = self.accept_any(&['+', '-',], options.clone(), None,) {
            let right = self.parse_term(options.clone(),)?;

            res = match op {
                '+' => RollAst::Add(Box::new(res,), Box::new(right,),),
                '-' => RollAst::Sub(Box::new(res,), Box::new(right,),),
                _ => unreachable!(),
            }
        }

        Ok(res,)
    }

    pub fn parse_term(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        let mut res = self.parse_factor(options.clone(),)?;

        loop {
            let mut options = options.clone();
            let opres = self.accept_any(&['*', '/',], options.clone(), None,);
            let mut op = if let Ok(i,) = opres {
                i
            } else if self.accept_string("mod", &options,).is_ok() {
                '%'
            } else {
                options.add_value("mod",).add_value("//",);
                break;
            };

            if op == '/' && self.accept('/', &options,).is_ok() {
                op = 'i'
            } else {
                options = options.add_value('/',);
            }

            let right = self.parse_factor(options,)?;

            res = match op {
                '*' => RollAst::Mul(Box::new(res,), Box::new(right,),),
                '/' => RollAst::Div(Box::new(res,), Box::new(right,),),
                'i' => RollAst::IDiv(Box::new(res,), Box::new(right,),),
                '%' => RollAst::Mod(Box::new(res,), Box::new(right,),),
                _ => unreachable!(),
            }
        }

        Ok(res,)
    }

    pub fn parse_factor(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        let backup = self.backup();

        Ok(match self.accept('-', &options,) {
            Ok(_,) => RollAst::Minus(Box::new(self.parse_power(options,)?,),),
            Err(o,) => {
                self.restore(backup,);

                return self.parse_power(o,);
            }
        },)
    }

    pub fn parse_power(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        let mut res = self.parse_atom(options.clone(),)?;
        if self.accept_string("**", &options,).is_ok() {
            let right = self.parse_factor(options,)?;
            res = RollAst::Power(Box::new(res,), Box::new(right,),);
        }

        Ok(res,)
    }

    pub fn parse_atom(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        let backup = self.backup();
        Ok(match self.parse_dice(options,) {
            Err(mut o,) => {
                self.restore(backup,);

                let backup = self.backup();
                if self.accept('(', &o,).is_ok() {
                    let sm = self.parse_sum(&o,)?;
                    self.accept(')', &o,)
                        .map_err(|e| e.message("missing closing parenthesis",),)?;

                    return Ok(sm,);
                }
                o = o.add_value('(',).message("tried to parse expression between parenthesis",);
                self.restore(backup,);

                self.parse_number(&o.message("tried to parse dice roll",),)?
            }
            Ok(i,) => i,
        },)
    }

    pub fn parse_dice(&mut self, mut options: RollOptions,) -> Result<RollAst, RollOptions,> {
        let backup = self.backup();

        let rolls = if self.advanced && self.accept('(', &options,).is_ok() {
            let sm = self.parse_sum(&options,)?;
            self.accept(')', &options,)
                .map_err(|e| e.message("missing closing parenthesis",),)?;

            Some(Box::new(sm,),)
        } else {
            if self.advanced {
                options = options
                    .add_value('(',)
                    .message("tried to parse expression between parenthesis",);
            }
            self.restore(backup,);
            self.parse_number(&options,).map(Box::new,).ok()
        };

        self.accept('d', &options,)?;
        let dpos = self.pos - 1;

        let backup = self.backup();
        let sides = if self.advanced && self.accept('(', &options,).is_ok() {
            let sm = self.parse_sum(&options,)?;
            self.accept(')', &options,)
                .map_err(|e| e.message("missing closing parenthesis",),)?;

            Some(Box::new(sm,),)
        } else {
            if self.advanced {
                options = options
                    .add_value('(',)
                    .message("tried to parse expression between parenthesis",);
            }
            self.restore(backup,);

            self.parse_number_or_percent(options.clone(),).map(Box::new,).ok()
        };

        let fm = if self.accept_string("kh", &options,).is_ok() || self.accept('h', &options,).is_ok() {
            FilterModifier::KeepHighest(Box::new(
                self.parse_number(&options,)
                    .unwrap_or_else(|_| RollAst::Const("1".to_string(),),),
            ),)
        } else if self.accept_string("dl", &options,).is_ok() || self.accept('l', &options,).is_ok() {
            FilterModifier::DropLowest(Box::new(
                self.parse_number(&options,)
                    .unwrap_or_else(|_| RollAst::Const("1".to_string(),),),
            ),)
        } else if self.accept_string("dh", &options,).is_ok() {
            FilterModifier::DropHighest(Box::new(
                self.parse_number(&options,)
                    .unwrap_or_else(|_| RollAst::Const("1".to_string(),),),
            ),)
        } else if self.accept_string("kl", &options,).is_ok() {
            FilterModifier::KeepLowest(Box::new(
                self.parse_number(&options,)
                    .unwrap_or_else(|_| RollAst::Const("1".to_string(),),),
            ),)
        } else {
            FilterModifier::None
        };

        Ok(RollAst::Dice(rolls, sides, fm, dpos,),)
    }

    pub fn parse_number_or_percent(&mut self, options: RollOptions,) -> Result<RollAst, RollOptions,> {
        if self.accept('%', &options,).is_ok() {
            Ok(RollAst::Const("100".to_ascii_lowercase(),),)
        } else {
            self.parse_number(&options.add_value('%',),)
        }
    }

    pub fn parse_number(&mut self, options: &RollOptions,) -> Result<RollAst, RollOptions,> {
        const DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.',];
        let digits_name = RollOptions::new("".to_string(),).add_value("0-9",);

        let mut number = vec![self
            .accept_any(DIGITS, options.clone(), Some(digits_name.clone(),),)
            .map_err(|e| options.clone().merge(e,).add_value('(',).message("tried to parse a number",),)?];

        loop {
            let backup = self.backup();
            if let Ok(digit,) = self.accept_any(DIGITS, options.clone(), Some(digits_name.clone(),),) {
                number.push(digit,);
            } else {
                self.restore(backup,);
                break;
            }
        }

        let string: String = number.iter().collect();

        Ok(RollAst::Const(string,),)
    }
}
