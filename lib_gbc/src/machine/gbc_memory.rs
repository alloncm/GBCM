use machine::memory;

const MEMORY_SIZE:u16 = 0xFFFF;

pub struct GbcMemory{
    memory: [u8;MEMORY_SIZE]
}

impl Memory for GbcMemory{
    pub fn read(&self, address:u16)->u8{
        return memory[address];
    }

    pub fn write(&mut self, address:u16, value:u8){
        memory[address] = value;
    }
}