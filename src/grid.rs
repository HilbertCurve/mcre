use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::mem;

use crate::block::{Block, BlockState};

#[derive(Debug)]
pub struct Grid {
    blocks: Vec<Vec<Vec<Block>>>,
    x_len: u32,
    y_len: u32,
    z_len: u32,
}

/* file format specs in bytes:
 *
 * Bytes 1-4: these spell out "mcrs" in ascii char codes
 * Bytes 5-12: width, height, and depth (x, y, z) dimension lengths, as ints
 * Bytes 13-end: data
 * Data packets consist of:
 * First byte: BlockStateID, see src/block.rs
 * Next bytes: encode block state, length dependent on value of first byte
 */

impl Grid {
    const HEADER: [u8; 4] = [b'm', b'c', b'r', b's'];

    pub fn new() -> Grid {
        Grid {
            blocks: vec![vec![vec![]]],
            x_len: 0,
            y_len: 0,
            z_len: 0,
        }
    }

    pub fn resize(&mut self, x_len: u32, y_len: u32, z_len: u32) {
        self.blocks = vec![vec![vec![
            Block::new(BlockState::NonBlock); 
        x_len as usize]; y_len as usize]; z_len as usize];

        self.x_len = x_len;
        self.y_len = y_len;
        self.z_len = z_len;
    }

    pub fn get(&mut self, x: u32, y: u32, z: u32) -> &Block {
        &self.blocks[z as usize][y as usize][x as usize]
    }

    fn get_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Block {
        &mut self.blocks[z as usize][y as usize][x as usize]
    }

    pub fn write(&self, fp: &str) -> io::Result<()> {
        let f = File::create(fp)?;

        {
            let mut buf = BufWriter::new(f);
            let dimensions: [u32; 3] = [self.x_len, self.y_len, self.z_len];

            buf.write_all(&Grid::HEADER)?;
            // interesting transmute hmm
            unsafe { buf.write_all(&mem::transmute::<[u32; 3], [u8; 12]>(dimensions))?; }

            for x in &self.blocks {
                for y in x {
                    for block in y {
                        let bytes = block.get_byte_repr();
                        buf.write_all(&bytes)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn read(&mut self, fp: &str) -> Result<(), Box<dyn Error>> {
        let mut f = File::open(fp)?;
        let md = fs::metadata(fp)?;

        let mut buf = vec![0u8; md.len() as usize];
        f.read(&mut buf)?;

        if &buf[0..4] != Grid::HEADER {
            println!("{:?}", &buf[0..4]);
            // return Err()
            panic!("how do you return proper errors?");
        }

        // shouldn't error
        let slice: &[u8; 12] = &buf[4..16].try_into()?;

        let dimensions = unsafe {
            mem::transmute::<&[u8; 12], &[u32; 3]>(slice)
        };

        self.resize(dimensions[0], dimensions[1], dimensions[2]);

        // remove header from current buffer
        buf.drain(..16);

        for i in 0..self.z_len {
            for j in 0..self.y_len {
                for k in 0..self.x_len {
                    // store integer value into each item
                    let consumed = self.get_mut(k, j, i).try_set(&buf)?;
                    buf.drain(..consumed);
                }
            }
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn tick(&self) {
        unimplemented!()
    }
}

