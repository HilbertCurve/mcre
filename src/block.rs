use enumflags2::{bitflags, BitFlags};

use std::convert::{TryFrom};

use crate::utils::ParseError;

/// Direction: general purpose orientation flags
///
/// Follows a left-handed rule: 
///     +x is right, -x is left,
///     +y is up, -y is down,
///     +z is forward, -z is backwards
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy)]
enum Dir {
    Up       = 0b00000001,
    Down     = 0b00000010,
    Left     = 0b00000100,
    Right    = 0b00001000,
    Forward  = 0b00010000,
    Backward = 0b00100000,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum PowerState {
    Off = 0,
    Weak,
    Strong,
}

impl TryFrom<u8> for PowerState {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<PowerState, ParseError> {
        match val {
            0 => Ok(PowerState::Off),
            1 => Ok(PowerState::Weak),
            2 => Ok(PowerState::Strong),
            _ => Err(ParseError::from("could not convert val into PowerState")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum BlockState {
    /// power level, direction 1 | direction 2 (could be just one bit)
    Redstone(u8, BitFlags<Dir>),
    /// activated, head pointing direction
    PistonBase(bool, BitFlags<Dir>),
    /// direction pushing block
    PistonHead(BitFlags<Dir>),
    /// power state, see PowerState
    Opaque(PowerState),
    /// doesn't really interact much
    Transparent,
    NonBlock,
}

#[repr(u8)]
enum BlockStateID {
    Redstone = 0u8,
    PistonBase,
    PistonHead,
    Opaque,
    Transparent,
    NonBlock,
}

// TODO: implement the following:
//  Redstone torches
//  Repeaters
//  Comparators
//  Block entities
//
impl TryFrom<u8> for BlockStateID {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<BlockStateID, ParseError> {
        match val {
            0 => Ok(BlockStateID::Redstone),
            1 => Ok(BlockStateID::PistonBase),
            2 => Ok(BlockStateID::PistonHead),
            3 => Ok(BlockStateID::Opaque),
            4 => Ok(BlockStateID::Transparent),
            5 => Ok(BlockStateID::NonBlock),
            _ => Err(ParseError::from("error converting val into BlockStateID"))
        }
    }
}

#[derive(Clone)]
pub struct Block {
    pub state: BlockState,
    // TODO: get references to parent grid working
}

impl Block {
    pub fn new(state: BlockState) -> Block {
        // i hope this doesn't take ownership
        Block { state }
    }

    pub fn default() -> Block {
        Block { state: BlockState::NonBlock }
    }

    pub fn update(&mut self) {
        // TODO: match each state with appropriate hard-coded update call
        match &self.state {
            BlockState::Redstone(_, _) => {}
            BlockState::PistonBase(_, _) => {}
            BlockState::PistonHead(_) => {}
            BlockState::Opaque(_) => {}
            _ => {}
        }
    }

    /// Attempts to mutate a block into a certain BlockState given a
    /// string of byte-encoded data. Returns a parse error if failure occurs.
    pub fn try_set(&mut self, slice: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let mut ret_offset = 0;
        let state: BlockState;
        let vec: Vec<u8> = slice.to_vec();

        let id = match BlockStateID::try_from(vec[0]) {
            Ok(val) => val,
            Err(_) => return Err(ParseError::boxed("invalid BlockStateID")),
        };

        state = match id {
            BlockStateID::Redstone => {
                let power: u8 = vec[1];
                let dirs: Result<BitFlags<Dir>, _> = vec[2].try_into();
                let dirs = match dirs {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("invalid redstone directions")),
                };

                // TODO: further redstone direction checking

                ret_offset = 3;

                BlockState::Redstone(power, dirs)
            }
            BlockStateID::PistonBase => {
                let on: bool = if vec[1] == 0 {
                    false
                } else if vec[1] == 1 {
                    true
                } else {
                    return Err(ParseError::boxed("invalid piston base state"))
                };

                let dir: Result<BitFlags<Dir>, _> = vec[2].try_into();
                let dir = match dir {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("invalid piston direction"))
                };

                // TODO: ensure piston has only one direction

                BlockState::PistonBase(on, dir)
            }
            BlockStateID::PistonHead => {
                let dir: Result<BitFlags<Dir>, _> = vec[2].try_into();
                let dir = match dir {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("invalid piston direction"))
                };

                ret_offset = 3;

                BlockState::PistonHead(dir)
            }
            BlockStateID::Opaque => {
                let state: Result<PowerState, _> = vec[1].try_into();
                let state = match state {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("could not convert into power state"))
                };

                ret_offset = 2;

                BlockState::Opaque(state as PowerState)
            }
            BlockStateID::Transparent => {
                ret_offset = 1;

                BlockState::Transparent
            }
            BlockStateID::NonBlock => {
                ret_offset = 1;
                
                BlockState::NonBlock
            }
        };

        self.state = state;

        Ok(ret_offset)
    }

    /// Converts its BlockState into a string of byte-encoded data for
    /// serialization in intermediate .mcrs filetype.
    pub fn get_byte_repr(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        match &self.state {
            BlockState::Redstone(power, dirs) => {
                ret.push(BlockStateID::Redstone as u8);
                ret.push(*power);
                ret.push(dirs.bits());
            }
            BlockState::PistonBase(on, dir) => {
                ret.push(BlockStateID::PistonBase as u8);
                ret.push(*on as u8);
                ret.push(dir.bits());
            }
            BlockState::PistonHead(dir) => {
                ret.push(BlockStateID::PistonHead as u8);
                ret.push(dir.bits());
            }
            BlockState::Opaque(state) => {
                ret.push(BlockStateID::Opaque as u8);
                ret.push(*state as u8);
            }
            BlockState::Transparent => ret.push(BlockStateID::Transparent as u8),
            BlockState::NonBlock => ret.push(BlockStateID::NonBlock as u8),
        }

        ret
    }
}

