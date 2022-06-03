use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct ParseError {
    reason: String,
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "error parsing values: {}", self.reason)
    }
}

impl Error for ParseError { }

impl ParseError {
    pub fn boxed(reason: &str) -> Box<ParseError> {
        Box::new(ParseError { reason: String::from(reason) })
    }
    pub fn from(reason: &str) -> ParseError {
        ParseError { reason: String::from(reason) }
    }
}

pub mod math {
    use super::ParseError;

    /// input is u64 because upcasting isn't lossy (even if it's probably not necessary)
    pub fn to_bool(val: u64) -> Result<bool, ParseError> {
        if val == 0 {
            Ok(false)
        } else if val == 1 {
            Ok(true)
        } else {
            Err(ParseError::from("invalid number, cannot convert to bool"))
        }
    }
}

// TODO: better file io; writing binary files

pub mod binio {
    use std::fs::File;
    use std::io::{self, BufWriter, Write, Read};
    use std::mem;
    use std::error::Error;
    use std::slice;

    pub fn write<T>(val: &T, writer: &mut BufWriter<File>) -> io::Result<()> {
        let size = mem::size_of::<T>();

        let ptr: *const _ = &val;
        let ptr = ptr as *mut u8;

        // hmmmm
        let bytes = unsafe {
            slice::from_raw_parts(ptr, size)
        };

        writer.write_all(bytes)?;
        Ok(())
    }

    // more of a debug method, not very fast, but used for directly
    // constructing .mcrs files
    pub fn write_bytes(bytes: &[u8], fp: &str) -> io::Result<()> {
        let file: File = File::create(fp)?;

        {
            let mut buf = BufWriter::new(file);
            buf.write_all(bytes)?;
        }

        Ok(())
    }

    pub fn read<T>(dest: &mut T, file: File, offset: usize) -> Result<(), Box<dyn Error>> {
        let mut iter = file.bytes();

        for _ in 0..offset {
            iter.next();
        }

        /*
        match iter.next() {
            panic!("file too small, TODO: formatting this error better")
        }
        */

        let size = mem::size_of::<T>();

        Ok(())
    }
}

