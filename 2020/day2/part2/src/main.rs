use std::error::Error;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

#[macro_use]
extern crate simple_error;

struct Policy {
    min: usize, max: usize,
    letter: char,
}

impl FromStr for Policy {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Policy, Box<dyn Error>> {
        let s1 = s.split(" ").collect::<Vec<&str>>();
        if s1.len() != 2 {
            bail!("could not parse {}", s);
        }

        let letter = match s1[1].chars().nth(0) {
            Some(c) => c,
            None => bail!("couldn't parse letter {}", s1[1])
        };

        let range = s1[0].split("-").collect::<Vec<&str>>();
        if range.len() != 2 {
            bail!("could not parse range {}", s1[0]);
        }

        Ok(Policy{
            min: range[0].parse()?,
            max: range[1].parse()?,
            letter: letter,
        })
    }
}

impl Policy {
    fn valid(&self, password: &str) -> bool {
        let min_match = match password.chars().nth(self.min-1) {
            Some(c) => c == self.letter,
            None => false,
        };
        
        let max_match = match password.chars().nth(self.max-1) {
            Some(c) => c == self.letter,
            None => false,
        };

        min_match ^ max_match
    }
}

struct PasswordLine {
    policy: Policy,
    password: String,
}

impl FromStr for PasswordLine {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<PasswordLine, Box<dyn Error>> {
        let line = s.split(": ").collect::<Vec<&str>>();
        if line.len() != 2 {
            bail!("could not parse {}", s);
        }

        let policy = line[0].parse::<Policy>()?;
        let password = line[1].to_string();

        Ok(PasswordLine{
            policy: policy,
            password: password,
        })
    }
}

impl PasswordLine {
    fn valid(&self) -> bool {
        self.policy.valid(&self.password)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input");
    let file = File::open(&path)?;
    let lines = BufReader::new(file).lines();
    let mut result = 0;
    for line in lines {
        let pl = line?.parse::<PasswordLine>()?;
        
        if pl.valid() {
            result += 1;
        }
    }
    println!("{}", result);
    Ok(())
}
