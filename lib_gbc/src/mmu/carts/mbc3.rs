use super::mbc::*;

const RAM_TIMER_ENABLE_VALUE:u8 = 0xA;
const EXTERNAL_RAM_READ_ERROR_VALUE:u8 = 0xFF;

pub struct Mbc3{
    program:Vec<u8>,
    ram:Vec<u8>,
    current_bank:u8, 
    ram_timer_enable:u8,
    ram_rtc_select:u8,
    latch_clock_data:u8,
    rtc_registers:[u8;4]
}

impl Mbc for Mbc3{
    fn read_bank0(&self, address:u16)->u8{
        self.program[address as usize]
    }

    fn read_current_bank(&self, address: u16)->u8{
        let current_bank = self.get_current_rom_bank() as u16;
        let internal_address:u16 = (RAM_BANK_SIZE * current_bank) + address;

        self.program[internal_address as usize]
    }

    fn write_rom(&mut self, address: u16, value: u8){
        match address{
            0..=0x1FFF=>self.ram_timer_enable = value,
            0x2000..=0x3FFF=>self.current_bank = value,
            0x4000..=0x5FFF=>self.ram_rtc_select = value,
            0x6000..=0x7FFF=>self.latch_clock_data = value,
            _=>std::panic!("cannot write to this address in mbc3 cartridge")
        }
    }

    fn read_external_ram(&self, address: u16)->u8{
        if self.ram_timer_enable != RAM_TIMER_ENABLE_VALUE{
            return EXTERNAL_RAM_READ_ERROR_VALUE;
        }
        
        return match self.ram_rtc_select{
            0..=3=>{
                let internal_address:u16 = self.ram_rtc_select as u16 * RAM_BANK_SIZE +  address;
                return self.ram[internal_address as usize];
            },
            0x8..=0xC=>self.rtc_registers[self.ram_rtc_select as usize],
            _=>EXTERNAL_RAM_READ_ERROR_VALUE
        };
    }

    fn write_external_ram(&mut self, address: u16, value: u8){
        if self.ram_timer_enable == RAM_TIMER_ENABLE_VALUE{
            match self.ram_rtc_select{
                0..=3=>{
                    let internal_address:u16 = self.ram_rtc_select as u16 * RAM_BANK_SIZE +  address;
                    self.ram[internal_address as usize] = value;
                },
                0x8..=0xC=>self.rtc_registers[self.ram_rtc_select as usize] = value,
                _=>{}
            }
        }
    }
}

impl Mbc3{

    pub fn new(program:Vec<u8>)->Self{
        let mut mbc = Mbc3{
            current_bank:0,
            latch_clock_data:0,
            program:program,
            ram:Vec::new(),
            ram_rtc_select:0,
            ram_timer_enable:0,
            rtc_registers:[0;4]
        };
        mbc.init();

        mbc
    }

    fn init(&mut self){
        let ram_index = self.program[MBC_RAM_SIZE_LOCATION];
        let ram_size = match ram_index{
            0=>0,
            1=>0x800,
            2=>0x2000,
            3=>0x8000,
            4=>0x20000,
            5=>0x10000,
            _=>std::panic!("no ram size in mbc3 cartridge")
        };

        self.ram = vec![0;ram_size as usize];
    }

    fn get_current_rom_bank(&self)->u8{
        //discard last bit as this register is 7 bits long
        let mut value = (self.current_bank << 1) >> 1;
        if value == 0{
            value += 1;
        }

        value
    }
}