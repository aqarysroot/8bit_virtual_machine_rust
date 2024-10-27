
struct CPU {
    a: u8,
    b: u8,
    pc: u16, 
    flag_z: bool,
    flag_c: bool,
}

impl CPU{
    fn new() -> Self {
        CPU {a:0, b:0, pc:0, flag_z:false, flag_c:false}
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.pc = 0;
        self.flag_z = false;
        self.flag_c = false;
    }
}

struct Memory {
    data: Vec<u8>
}

impl Memory {
    fn new(size: usize) -> Self {
        Memory {data: vec![0; size]}
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

enum Instruction {
    LDA(u8),
    ADD(u8),
    STA(u16),
    OUT,
    HLT,
}

impl CPU {
    fn fetch(&mut self, memory: &Memory) -> Option<Instruction> {
        let opcode = memory.read(self.pc);
        self.pc += 1;
        self.map_opcode_to_instruction(opcode, memory)
    }

    fn map_opcode_to_instruction(&mut self, opcode: u8, memory: &Memory) -> Option<Instruction> {
        match opcode {
            0x01 => { // LDA
                let value = memory.read(self.pc);
                self.pc += 1;
                Some(Instruction::LDA(value))
            }
            0x02 => { // ADD
                let value = memory.read(self.pc);
                self.pc += 1;
                Some(Instruction::ADD(value))
            }
            0x03 => { // STA
                let addr = ((memory.read(self.pc) as u16) << 8) | memory.read(self.pc + 1) as u16;
                self.pc += 2;
                Some(Instruction::STA(addr))
            }
            0xFF => Some(Instruction::OUT), // No additional argument needed
            0x00 => Some(Instruction::HLT), // No additional argument needed
            _ => {
                println!("Unknown instruction: 0x{:02X}", opcode);
                None
            }
        }
    }

    fn decode_and_execute(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction {
            Instruction::LDA(value) => {
                self.a = value;
            }
            Instruction::ADD(value) => {
                let (result, carry) = self.a.overflowing_add(value);
                self.a = result;
                self.flag_c = carry;
                self.flag_z = self.a == 0;
            }
            Instruction::STA(address) => {
                memory.write(address, self.a);
            }
            Instruction::OUT => {
                println!("Output: {}", self.a);
            }
            Instruction::HLT => {
                println!("Program halted.");
                self.pc = memory.data.len() as u16;
            }
        }
    }

    fn run(&mut self, memory: &mut Memory) {
        loop {
           match self.fetch(memory) {
            Some(instruction) => self.decode_and_execute(instruction, memory),
            None => break,
           }
        }
    }

}

fn main() {
    let mut memory = Memory::new(256);
    let mut cpu = CPU::new();

    memory.write(0, 0x01);
    memory.write(1, 5);
    memory.write(2, 0x02);
    memory.write(3, 3);
    memory.write(4, 0xFF);
    memory.write(5, 0x00);

    cpu.run(&mut memory);
    cpu.reset();

}


