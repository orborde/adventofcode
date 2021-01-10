use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

#[macro_use]
extern crate simple_error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input");
    let file = File::open(&path)?;

    let mut treemap = Vec::new();
    for line in BufReader::new(file).lines() {
        let data: Vec<u8> = line?.into();
        treemap.push(data);
    }

    let mut product: u32 = 1;
    for (dc, dr) in vec![(1,1), (3,1), (5,1), (7,1), (1,2)] {
        let mut row: usize = 0;
        let mut col: usize = 0;
        let mut trees: u32 = 0;
        while row < treemap.len() {
            let rowdata = &treemap[row];
            match rowdata[col % rowdata.len()] {
                b'#' => { trees = trees.checked_add(1).unwrap() },
                b'.' => (),
                x => bail!("unknown char {}", x)
            }

            row = row.checked_add(dr).unwrap();
            col = col.checked_add(dc).unwrap();
        }

        println!("right {} down {} -> {}", dr, dc, trees);
        product = product.checked_mul(trees).unwrap();
    }

    println!("{}", product);
    Ok(())
}
