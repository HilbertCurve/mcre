mod grid;
mod block;

use std::error::Error;

use crate::grid::Grid;

fn main() -> Result<(), Box<dyn Error>> {
    let mut g = Grid::new(1, 2, 3);
    g.write("/tmp/lmao")?;
    g.read("/tmp/lmao")?;

    assert!(g.x_len == 1, "bad x after read");
    assert!(g.y_len == 2, "bad y after read");
    assert!(g.z_len == 3, "bad z after read");

    Ok(())
}

