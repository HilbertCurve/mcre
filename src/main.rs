mod grid;
mod block;
mod utils;

use std::error::Error;

use crate::grid::*;
use crate::block::*;
use crate::utils::ParseError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Grid::new();

    grid.resize(2, 3, 1);
    grid.write("/tmp/test_world")?;
    grid.read("/tmp/test_world")?;
    grid.write("/tmp/test_world1")?;


    Ok(())
}

