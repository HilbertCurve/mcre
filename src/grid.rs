use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::mem;

use crate::block::Block;

pub struct Grid {
    grid: Vec<Vec<Vec<Block>>>,
    pub x_len: u32,
    pub y_len: u32,
    pub z_len: u32,
}

impl Grid {
    const HEADER: [u8; 4] = ['m' as u8, 'c' as u8, 'r' as u8, 's' as u8];

    pub fn new(x_len: u32, y_len: u32, z_len: u32) -> Grid {
        Grid {
            grid: vec![vec![vec![Block::new(0); z_len as usize]; y_len as usize]; x_len as usize],
            x_len, y_len, z_len
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &Block {
        &self.grid[x][y][z]
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Block {
        &mut self.grid[x][y][z]
    }

    /* file format specs in bytes:
     *
     * Bytes 1-4: these spell out "mcrs" in ascii char codes
     * Bytes 5-12: width, height, and depth (x, y, z) dimension lengths, as ints
     * Bytes 13-end: data
     */
    pub fn write(&self, fp: &str) -> io::Result<()> {
        let f = File::create(fp)?;

        {
            let mut buf = BufWriter::new(f);
            let dimensions: [u32; 3] = [self.x_len, self.y_len, self.z_len];

            buf.write_all(&Grid::HEADER)?;
            // interesting transmute hmm
            unsafe { buf.write_all(&mem::transmute::<[u32; 3], [u8; 12]>(dimensions))?; }

            for x in &self.grid {
                for y in x {
                    for block in y {
                        let bytes = unsafe {
                            mem::transmute::<u32, [u8; 4]>(block.id)
                        };
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

        // TODO: populate arrays

        self.x_len = dimensions[0];
        self.y_len = dimensions[1];
        self.z_len = dimensions[2];

        Ok(())
    }

    #[allow(unused)]
    pub fn tick(&self) {
        unimplemented!()
    }
}

