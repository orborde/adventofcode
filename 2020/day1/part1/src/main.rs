use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[macro_use]
extern crate simple_error;


fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input");
    let file = File::open(&path)?;
    let lines = BufReader::new(file).lines();

    let (x,y) = || -> Result<(i32,i32), Box<dyn Error>> {
        let mut items = HashSet::new();

        for line_maybe in lines {
            let line = line_maybe?; // wat
            let v = line.parse::<i32>()?;
            let pair = 2020 - v;
            if items.contains(&pair) {
                return Ok((v,pair))
            }

            items.insert(v);
        }

        bail!("no matching pair found")
    }()?;

    println!("{} {} -> {}", x, y, x*y);

    Ok(())
}
