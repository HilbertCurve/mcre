// might turn Block into a trait
pub struct Block {
    pub id: u32,
}

impl Block {
    pub fn new(id: u32) -> Block {
        Block { id }
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        Block { id: self.id }
    }
}

