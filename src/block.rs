use enumflags2::{bitflags, BitFlags};

use std::convert::{TryFrom};

use crate::utils::{ParseError, math};

/// Direction: general purpose orientation flags
///
/// Follows a left-handed rule: 
///     +x is right, -x is left,
///     +y is up, -y is down,
///     +z is forward, -z is backwards
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up       = 0b00000001,
    Down     = 0b00000010,
    Left     = 0b00000100,
    Right    = 0b00001000,
    Forward  = 0b00010000,
    Backward = 0b00100000,
}

impl TryFrom<u8> for Dir {
    type Error = ParseError;
    fn try_from(val: u8) -> Result<Dir, Self::Error> {
        match val {
            0b00000001 => Ok(Dir::Up),
            0b00000010 => Ok(Dir::Down),
            0b00000100 => Ok(Dir::Left),
            0b00001000 => Ok(Dir::Right),
            0b00010000 => Ok(Dir::Forward),
            0b00100000 => Ok(Dir::Backward),
            _ => Err(ParseError::from("could not convert val into Dir")),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum PowerState {
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

#[derive(Clone, Copy, Debug)]
pub enum BlockState {
    /// power level, direction 1 | direction 2 (could be just one bit)
    Redstone(u8, BitFlags<Dir>),
    /// activated, position of base of torch (which block it's haning off of)
    Torch(bool, Dir),
    /// delay length, activated?, locked?, forward pointing direction
    Repeater(u8, bool, bool, Dir),
    /// power level, compare/subtract, forward pointing direction
    Comparator(u8, bool, Dir),
    /// activated, head pointing direction
    PistonBase(bool, Dir),
    /// sticky?, direction pushing block
    PistonHead(bool, Dir),
    /// power state; see PowerState, color
    Opaque(PowerState, u8),
    /// power level (if pulled from comparator)
    BlockEntity(u8),
    /// doesn't really interact much
    Transparent,
    /// air or structure void (maybe)
    NonBlock,
}

#[repr(u8)]
#[derive(Debug)]
enum BlockStateID {
    Redstone = 0u8,
    Torch,
    Repeater,
    Comparator,
    PistonBase,
    PistonHead,
    Opaque,
    BlockEntity,
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
        // could probably do a range-checked mem::transmute here??
        match val {
            0 => Ok(BlockStateID::Redstone),
            1 => Ok(BlockStateID::Torch),
            2 => Ok(BlockStateID::Repeater),
            3 => Ok(BlockStateID::Comparator),
            4 => Ok(BlockStateID::PistonBase),
            5 => Ok(BlockStateID::PistonHead),
            6 => Ok(BlockStateID::Opaque),
            7 => Ok(BlockStateID::BlockEntity),
            8 => Ok(BlockStateID::Transparent),
            9 => Ok(BlockStateID::NonBlock),
            _ => Err(ParseError::from("error converting val into BlockStateID"))
        }
    }
}

#[derive(Clone, Debug)]
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
            BlockState::Redstone(..) => {}
            BlockState::Torch(..) => {}
            BlockState::Repeater(..) => {}
            BlockState::Comparator(..) => {}
            BlockState::PistonBase(..) => {}
            BlockState::PistonHead(..) => {}
            BlockState::Opaque(..) => {}
            BlockState::BlockEntity(..) => {}
            _ => {}
        }
    }

    /// Attempts to mutate a block into a certain BlockState given a
    /// string of byte-encoded data. Returns a parse error if failure occurs.
    pub fn try_set(&mut self, slice: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let ret_offset;
        let state: BlockState;
        let vec: Vec<u8> = slice.to_vec();

        let id = match BlockStateID::try_from(vec[0]) {
            Ok(val) => val,
            Err(_) => return Err(ParseError::boxed("invalid BlockStateID")),
        };

        state = match id {
            BlockStateID::Redstone => {
                let power: u8 = vec[1];
                println!("{:b}", vec[2]);
                let dirs: Result<BitFlags<Dir>, _> = vec[2].try_into();
                let dirs = match dirs {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("invalid redstone directions")),
                };

                // TODO: further redstone direction checking

                ret_offset = 3;

                BlockState::Redstone(power, dirs)
            }
            BlockStateID::Torch => {
                let activated = math::to_bool(vec[1] as u64)?;
                let dir: Dir = vec[2].try_into()?;

                ret_offset = 3;

                BlockState::Torch(activated, dir)
            }
            BlockStateID::Repeater => {
                let delay = vec[1];
                let activated = math::to_bool(vec[2] as u64)?;
                let locked = math::to_bool(vec[3] as u64)?;
                let dir: Dir = vec[4].try_into()?;

                ret_offset = 5;

                BlockState::Repeater(delay, activated, locked, dir)
            }
            BlockStateID::Comparator => {
                let power = vec[1];
                let mode = math::to_bool(vec[2] as u64)?;
                let dir: Dir = vec[3].try_into()?;

                ret_offset = 4;

                BlockState::Comparator(power, mode, dir)
            }
            BlockStateID::PistonBase => {
                let on = math::to_bool(vec[1] as u64)?;
                let dir: Dir = vec[2].try_into()?;

                ret_offset = 3;

                BlockState::PistonBase(on, dir)
            }
            BlockStateID::PistonHead => {
                let sticky = math::to_bool(vec[1] as u64)?;
                let dir: Dir = vec[2].try_into()?;

                ret_offset = 3;

                BlockState::PistonHead(sticky, dir)
            }
            BlockStateID::Opaque => {
                let state: Result<PowerState, _> = vec[1].try_into();
                let state = match state {
                    Ok(val) => val,
                    Err(_) => return Err(ParseError::boxed("could not convert into power state"))
                };

                let color = vec[2];

                ret_offset = 3;

                BlockState::Opaque(state as PowerState, color)
            }
            BlockStateID::BlockEntity => {
                let power = vec[1];

                ret_offset = 2;

                BlockState::BlockEntity(power)
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
            BlockState::Torch(activated, dir) => {
                ret.push(BlockStateID::Torch as u8);
                ret.push(*activated as u8);
                ret.push(*dir as u8)
            }
            BlockState::Repeater(delay, activated, locked, dir) => {
                ret.push(BlockStateID::Repeater as u8);
                ret.push(*delay);
                ret.push(*activated as u8);
                ret.push(*locked as u8);
                ret.push(*dir as u8)
            }
            BlockState::Comparator(power, mode, dir) => {
                ret.push(BlockStateID::Comparator as u8);
                ret.push(*power);
                ret.push(*mode as u8);
                ret.push(*dir as u8)
            }
            BlockState::PistonBase(on, dir) => {
                ret.push(BlockStateID::PistonBase as u8);
                ret.push(*on as u8);
                ret.push(*dir as u8);
            }
            BlockState::PistonHead(sticky, dir) => {
                ret.push(BlockStateID::PistonHead as u8);
                ret.push(*sticky as u8);
                ret.push(*dir as u8);
            }
            BlockState::Opaque(state, color) => {
                ret.push(BlockStateID::Opaque as u8);
                ret.push(*state as u8);
                ret.push(*color);
            }
            BlockState::BlockEntity(power) => {
                ret.push(BlockStateID::BlockEntity as u8);
                ret.push(*power);
            }
            BlockState::Transparent => ret.push(BlockStateID::Transparent as u8),
            BlockState::NonBlock => ret.push(BlockStateID::NonBlock as u8),
        }

        ret
    }
}

