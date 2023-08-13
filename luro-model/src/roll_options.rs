use core::fmt;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RollOptions {
    options: HashSet<String>,
    lastpos: u64,
    messages: Vec<String>,
    source: String
}

impl RollOptions {
    pub fn new(source: String) -> Self {
        Self {
            options: Default::default(),
            lastpos: Default::default(),
            messages: Default::default(),
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

    pub fn merge(mut self, other: RollOptions) -> Self {
        for i in other.options {
            self = self.add_value(i);
        }

        self
    }

    pub fn add_value(mut self, value: impl Into<String>) -> Self {
        self.options.insert(value.into());
        self
    }
}

impl fmt::Display for RollOptions {
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
