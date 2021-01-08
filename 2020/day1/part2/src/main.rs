use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use itertools::Itertools;

#[macro_use]
extern crate simple_error;


fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input");
    let file = File::open(&path)?;
    let lines = BufReader::new(file).lines();
    let items = lines
        .map(|line| Ok(line?.parse::<i32>()?))
        .collect::<Result<Vec<i32>, Box<dyn Error>>>()?;
    let (x,y,z) = || -> Result<(i32,i32,i32), Box<dyn Error>> {
        for (a,b,c) in items.into_iter().tuple_combinations() {
            if a+b+c == 2020 {
                return Ok((a,b,c))
            }
        }

        bail!("no matching triple found")
    }()?;

    println!("{} {} {} -> {}", x, y, z, x*y*z);

    Ok(())
}
