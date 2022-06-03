mod grid;
mod block;
mod utils;

use std::error::Error;

use crate::grid::*;
use crate::utils::{binio};

//                header + data
const TEST_FILE: [u8; 16 + 30] = [
    b'm', b'c', b'r', b's',
    02, 00, 00, 00, 04, 00, 00, 00, 02, 00, 00, 00,
    00, 14, 12, 00, 15, 20,
    08,         06, 02, 01,
    09,         01, 01, 02,
    08,         08,

    09,         00, 14, 32 + 16,
    08,         08,
    09,         02, 01, 01, 00, 32,
    09,         08,
];

fn main() -> Result<(), Box<dyn Error>> {
    binio::write_bytes(&TEST_FILE, "/tmp/test_world")?;

    let mut grid = Grid::new();

    grid.read("/tmp/test_world")?;
    dbg!("{:?}", &grid);
    grid.write("/tmp/test_world2")?;

    Ok(())
}

