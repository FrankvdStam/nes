#[allow(dead_code)]
#[allow(non_snake_case)]

use crate::Bus;
use std::cell::RefCell;

pub struct Cpu
{    
    //I/O
	address: u16,
	data: u8,
	read: bool,
		
	//registers
	program_counter: u16,
	accumulator: u8,
	index_register_y: u8,
	index_register_x: u8,
	stack_pointer: u8,

	//Processor status is actually an 8bit register, I decided to write them out to fields
	carry: bool,
	zero: bool,
	interupt_request_disable: bool,
	decimal_mode: bool,
	break_command: bool,
	overflow: bool,
	negative: bool,


	instructions: [fn(&mut Cpu) -> u8; 16*16],
	addressing_modes: [fn(&mut Cpu) -> u8; 16*16],
	mnemonics: [&'static str; 16*16],

	bus: RefCell<Bus>
}
 

impl Cpu
{
    pub fn new(bus: RefCell<Bus>) -> Self 
    {
		let mut cpu = Cpu
        {
			address: 0,
			data: 0,
			read: true,

			program_counter: 0xfffc,

			accumulator: 0,
			index_register_y: 0,
			index_register_x: 0,
			stack_pointer: 0,
			carry: false,
			zero: false,
			interupt_request_disable: false,
			decimal_mode: false,
			break_command: false,
			overflow: false,
			negative: false,

			instructions: [Cpu::XXX; 16*16],
			addressing_modes: [Cpu::XXX; 16*16],
			mnemonics: [""; 16*16],

			bus: bus,
        };

		cpu.init_tables();
		cpu.reset();

		return cpu;
    }

	pub fn reset(&mut self)
	{
		self.address = 0;
		self.data = 0;
		self.read = true;
		
		self.program_counter = 0xfffc;
		
		self.accumulator = 0;
		self.index_register_y = 0;
		self.index_register_x = 0;
		self.stack_pointer = 0;
		self.carry = false;
		self.zero = false;
		self.interupt_request_disable = false;
		self.decimal_mode = false;
		self.break_command = false;
		self.overflow = false;
		self.negative = false;
	}

	pub fn set_address(&mut self, address: u16)
	{
		self.address = address;
		self.bus.borrow_mut().set_address(address);
	}

	pub fn set_data(&mut self, data: u8)
	{
		self.data = data;
	}

	pub fn get_program_counter(&self) -> u16
	{
		return self.program_counter;
	}

	pub fn clock_tick(&mut self)
	{
	}

	

	//=========================================================================================================================
	//Addressing modes
	//Each addressing mode returns a 0 or 1 corresponding to wether or not they may require another cpu cycle to complete the
	//full instruction.
	//=========================================================================================================================
	
	//Implied
	fn IMP(&mut self) -> u8
	{
		return 0;
	}

	//Immediate
	fn IMM(&mut self) -> u8
	{
		return 0;
	}

	//Zero-page
	fn ZP0(&mut self) -> u8
	{
		return 0;
	}

	//Zero page indexed (x)
	fn ZPX(&mut self) -> u8
	{
		return 0;
	}

	//Zero page indexed (y)
	fn ZPY(&mut self) -> u8
	{
		return 0;
	}

	//Relative
	fn REL(&mut self) -> u8
	{
		return 0;
	}

	//Absolute
	fn ABS(&mut self) -> u8
	{
		return 0;
	}
	
	//Absolute indexed (x)
	fn ABX(&mut self) -> u8
	{
		return 0;
	}

	//Absolute indexed (y)
	fn ABY(&mut self) -> u8
	{
		return 0;
	}

	//Indirect
	fn IND(&mut self) -> u8
	{
		return 0;
	}

	//Indirect zero page (x)
	fn IZX(&mut self) -> u8
	{
		return 0;
	}

	//Indirect zero page (y)
	fn IZY(&mut self) -> u8
	{
		return 0;
	}



	//=========================================================================================================================
	//Instructions
	//Each instruction returns a 0 or 1 corresponding to wether or not they may require another cpu cycle to complete the
	//full instruction.
	//=========================================================================================================================
			
	fn XXX(&mut self) -> u8
	{
		panic!("Unknown instruction or addressing mode encoutered.");
	}


	//Add Memory to Accumulator with Carry
	fn ADC(&mut self) -> u8
	{
		return 0;
	}

	//"AND" Memory with Accumulator
	fn AND(&mut self) -> u8
	{
		return 0;
	}

	//Shift left One Bit(Memory or Accumulator)
	fn ASL(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Carry Clear
	fn BCC(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Carry Set
	fn BCS(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Result Zero
	fn BEQ(&mut self) -> u8
	{
		return 0;
	}

	//Test Bits in Memory with Accumulator
	fn BIT(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Result Minus
	fn BMI(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Result not Zero
	fn BNE(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Result Plus
	fn BPL(&mut self) -> u8
	{
		return 0;
	}

	//Force Break
	fn BRK(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Overflow Clear
	fn BVC(&mut self) -> u8
	{
		return 0;
	}

	//Branch on Overflow Set
	fn BVS(&mut self) -> u8
	{
		return 0;
	}

	//Clear Carry Flag
	fn CLC(&mut self) -> u8
	{
		return 0;
	}

	//Clear Decimal Mode
	fn CLD(&mut self) -> u8
	{
		return 0;
	}

	//Clear Interrupt Disable Bit
	fn CLI(&mut self) -> u8
	{
		return 0;
	}

	//Clear Overflow Flag
	fn CLV(&mut self) -> u8
	{
		return 0;
	}

	//Compare Memory and Accumulator
	fn CMP(&mut self) -> u8
	{
		return 0;
	}

	//Compare Memory and Index X
	fn CPX(&mut self) -> u8
	{
		return 0;
	}

	//Compare Memory and Index Y
	fn CPY(&mut self) -> u8
	{
		return 0;
	}

	//Decrement Memory by One
	fn DEC(&mut self) -> u8
	{
		return 0;
	}


	//Decrement Index X by One
	fn DEX(&mut self) -> u8
	{
		return 0;
	}

	//Decrement Index Y by One
	fn DEY(&mut self) -> u8
	{
		return 0;
	}

	//"Exclusive-or" Memory with Accumulator
	fn EOR(&mut self) -> u8
	{
		return 0;
	}

	//Increment Memory by One
	fn INC(&mut self) -> u8
	{
		return 0;
	}

	//Increment Index X by One
	fn INX(&mut self) -> u8
	{
		return 0;
	}

	//Increment Index Y by One
	fn INY(&mut self) -> u8
	{
		return 0;
	}

	//Jump to New Location
	fn JMP(&mut self) -> u8
	{
		return 0;
	}

	//Jump to New Location Saving Return Address
	fn JSR(&mut self) -> u8
	{
		return 0;
	}

	//Load Accumulator with Memory
	fn LDA(&mut self) -> u8
	{
		return 0;
	}

	//Load Index X with Memory
	fn LDX(&mut self) -> u8
	{
		return 0;
	}

	//Load Index Y with Memory
	fn LDY(&mut self) -> u8
	{
		return 0;
	}

	//Shift One Bit Right(Memory or Accumulator)
	fn LSR(&mut self) -> u8
	{
		return 0;
	}

	//No Operation
	fn NOP(&mut self) -> u8
	{
		return 0;
	}

	//"OR" Memory with Accumulator
	fn ORA(&mut self) -> u8
	{
		return 0;
	}


	//Push Accumulator on Stack
	fn PHA(&mut self) -> u8
	{
		return 0;
	}

	//Push Processor Statuson Stack
	fn PHP(&mut self) -> u8
	{
		return 0;
	}

	//Pull Accumulator from Stack
	fn PLA(&mut self) -> u8
	{
		return 0;
	}

	//Pull Processor Status from Stack
	fn PLP(&mut self) -> u8
	{
		return 0;
	}

	//Rotate One Bit Left(Memory or Accumulator)
	fn ROL(&mut self) -> u8
	{
		return 0;
	}

	//Rotate One Bit Right(Memory or Accumulator)
	fn ROR(&mut self) -> u8
	{
		return 0;
	}

	//Return from Interrupt
	fn RTI(&mut self) -> u8
	{
		return 0;
	}

	//Return from Subroutine
	fn RTS(&mut self) -> u8
	{
		return 0;
	}

	//Subtract Memory from Accumulator with Borrow
	fn SBC(&mut self) -> u8
	{
		return 0;
	}

	//Set Carry Flag
	fn SEC(&mut self) -> u8
	{
		return 0;
	}

	//Set Decimal Mode
	fn SED(&mut self) -> u8
	{
		return 0;
	}

	//Set Interrupt Disable Status
	fn SEI(&mut self) -> u8
	{
		return 0;
	}

	//Store Accumulator in Memory
	fn STA(&mut self) -> u8
	{
		return 0;
	}

	//Store Index X in Memory
	fn STX(&mut self) -> u8
	{
		return 0;
	}

	//Store Index Y in Memory
	fn STY(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Accumulator to Index X
	fn TAX(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Accumulator to Index Y
	fn TAY(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Stack Pointer to Index
	fn TSX(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Index X to Accumulator
	fn TXA(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Index X to Stack Pointer
	fn TXS(&mut self) -> u8
	{
		return 0;
	}

	//Transfer Index Y to Accumulator
	fn TYA(&mut self) -> u8
	{
		return 0;
	}


	//=========================================================================================================================
	//Table initialization
	//This method sets up all the tables. It's big, it's long, it's boring but it's also safe and doesn't require 
	//references or lifetimes or any of that.
	//=========================================================================================================================


	fn init_tables(&mut self)
	{
		//=========================================================================================================================
		//Instructions
		//=========================================================================================================================

		self.instructions[0x00] = Cpu::BRK;
		self.instructions[0x01] = Cpu::ORA;
		self.instructions[0x02] = Cpu::XXX;
		self.instructions[0x03] = Cpu::XXX;
		self.instructions[0x04] = Cpu::XXX;
		self.instructions[0x05] = Cpu::ORA;
		self.instructions[0x06] = Cpu::ASL;
		self.instructions[0x07] = Cpu::XXX;
		self.instructions[0x08] = Cpu::PHP;
		self.instructions[0x09] = Cpu::ORA;
		self.instructions[0x0a] = Cpu::ASL;
		self.instructions[0x0b] = Cpu::XXX;
		self.instructions[0x0c] = Cpu::XXX;
		self.instructions[0x0d] = Cpu::ORA;
		self.instructions[0x0e] = Cpu::ASL;
		self.instructions[0x0f] = Cpu::XXX;
			 
		self.instructions[0x10] = Cpu::BPL;
		self.instructions[0x11] = Cpu::ORA;
		self.instructions[0x12] = Cpu::XXX;
		self.instructions[0x13] = Cpu::XXX;
		self.instructions[0x14] = Cpu::XXX;
		self.instructions[0x15] = Cpu::ORA;
		self.instructions[0x16] = Cpu::ASL;
		self.instructions[0x17] = Cpu::XXX;
		self.instructions[0x18] = Cpu::CLC;
		self.instructions[0x19] = Cpu::ORA;
		self.instructions[0x1a] = Cpu::XXX;
		self.instructions[0x1b] = Cpu::XXX;
		self.instructions[0x1c] = Cpu::XXX;
		self.instructions[0x1d] = Cpu::ORA;
		self.instructions[0x1e] = Cpu::ASL;
		self.instructions[0x1f] = Cpu::XXX;
			 
		self.instructions[0x20] = Cpu::JSR;
		self.instructions[0x21] = Cpu::AND;
		self.instructions[0x22] = Cpu::XXX;
		self.instructions[0x23] = Cpu::XXX;
		self.instructions[0x24] = Cpu::BIT;
		self.instructions[0x25] = Cpu::AND;
		self.instructions[0x26] = Cpu::ROL;
		self.instructions[0x27] = Cpu::XXX;
		self.instructions[0x28] = Cpu::PLP;
		self.instructions[0x29] = Cpu::AND;
		self.instructions[0x2a] = Cpu::ROL;
		self.instructions[0x2b] = Cpu::XXX;
		self.instructions[0x2c] = Cpu::BIT;
		self.instructions[0x2d] = Cpu::AND;
		self.instructions[0x2e] = Cpu::ROL;
		self.instructions[0x2f] = Cpu::XXX;
			 
		self.instructions[0x30] = Cpu::BMI;
		self.instructions[0x31] = Cpu::AND;
		self.instructions[0x32] = Cpu::XXX;
		self.instructions[0x33] = Cpu::XXX;
		self.instructions[0x34] = Cpu::XXX;
		self.instructions[0x35] = Cpu::AND;
		self.instructions[0x36] = Cpu::ROL;
		self.instructions[0x37] = Cpu::XXX;
		self.instructions[0x38] = Cpu::SEC;
		self.instructions[0x39] = Cpu::AND;
		self.instructions[0x3a] = Cpu::XXX;
		self.instructions[0x3b] = Cpu::XXX;
		self.instructions[0x3c] = Cpu::XXX;
		self.instructions[0x3d] = Cpu::AND;
		self.instructions[0x3e] = Cpu::ROL;
		self.instructions[0x3f] = Cpu::XXX;
			 
		self.instructions[0x40] = Cpu::RTI;
		self.instructions[0x41] = Cpu::EOR;
		self.instructions[0x42] = Cpu::XXX;
		self.instructions[0x43] = Cpu::XXX;
		self.instructions[0x44] = Cpu::XXX;
		self.instructions[0x45] = Cpu::EOR;
		self.instructions[0x46] = Cpu::LSR;
		self.instructions[0x47] = Cpu::XXX;
		self.instructions[0x48] = Cpu::PHA;
		self.instructions[0x49] = Cpu::EOR;
		self.instructions[0x4a] = Cpu::LSR;
		self.instructions[0x4b] = Cpu::XXX;
		self.instructions[0x4c] = Cpu::JMP;
		self.instructions[0x4d] = Cpu::EOR;
		self.instructions[0x4e] = Cpu::LSR;
		self.instructions[0x4f] = Cpu::XXX;
			 					 
		self.instructions[0x50] = Cpu::BVC;
		self.instructions[0x51] = Cpu::EOR;
		self.instructions[0x52] = Cpu::XXX;
		self.instructions[0x53] = Cpu::XXX;
		self.instructions[0x54] = Cpu::XXX;
		self.instructions[0x55] = Cpu::EOR;
		self.instructions[0x56] = Cpu::LSR;
		self.instructions[0x57] = Cpu::XXX;
		self.instructions[0x58] = Cpu::CLI;
		self.instructions[0x59] = Cpu::EOR;
		self.instructions[0x5a] = Cpu::XXX;
		self.instructions[0x5b] = Cpu::XXX;
		self.instructions[0x5c] = Cpu::XXX;
		self.instructions[0x5d] = Cpu::EOR;
		self.instructions[0x5e] = Cpu::LSR;
		self.instructions[0x5f] = Cpu::XXX;

		self.instructions[0x60] = Cpu::RTS;
		self.instructions[0x61] = Cpu::ADC;
		self.instructions[0x62] = Cpu::XXX;
		self.instructions[0x63] = Cpu::XXX;
		self.instructions[0x64] = Cpu::XXX;
		self.instructions[0x65] = Cpu::ADC;
		self.instructions[0x66] = Cpu::ROR;
		self.instructions[0x67] = Cpu::XXX;
		self.instructions[0x68] = Cpu::PLA;
		self.instructions[0x69] = Cpu::ADC;
		self.instructions[0x6a] = Cpu::ROR;
		self.instructions[0x6b] = Cpu::XXX;
		self.instructions[0x6c] = Cpu::JMP;
		self.instructions[0x6d] = Cpu::ADC;
		self.instructions[0x6e] = Cpu::ROR;
		self.instructions[0x6f] = Cpu::XXX;
			 					 
		self.instructions[0x70] = Cpu::BVS;
		self.instructions[0x71] = Cpu::ADC;
		self.instructions[0x72] = Cpu::XXX;
		self.instructions[0x73] = Cpu::XXX;
		self.instructions[0x74] = Cpu::XXX;
		self.instructions[0x75] = Cpu::ADC;
		self.instructions[0x76] = Cpu::ROR;
		self.instructions[0x77] = Cpu::XXX;
		self.instructions[0x78] = Cpu::SEI;
		self.instructions[0x79] = Cpu::ADC;
		self.instructions[0x7a] = Cpu::XXX;
		self.instructions[0x7b] = Cpu::XXX;
		self.instructions[0x7c] = Cpu::XXX;
		self.instructions[0x7d] = Cpu::ADC;
		self.instructions[0x7e] = Cpu::ROR;
		self.instructions[0x7f] = Cpu::XXX;
			 					  
		self.instructions[0x80] = Cpu::XXX;
		self.instructions[0x81] = Cpu::STA;
		self.instructions[0x82] = Cpu::XXX;
		self.instructions[0x83] = Cpu::XXX;
		self.instructions[0x84] = Cpu::STY;
		self.instructions[0x85] = Cpu::STA;
		self.instructions[0x86] = Cpu::STX;
		self.instructions[0x87] = Cpu::XXX;
		self.instructions[0x88] = Cpu::DEY;
		self.instructions[0x89] = Cpu::XXX;
		self.instructions[0x8a] = Cpu::TXA;
		self.instructions[0x8b] = Cpu::XXX;
		self.instructions[0x8c] = Cpu::STY;
		self.instructions[0x8d] = Cpu::STA;
		self.instructions[0x8e] = Cpu::STX;
		self.instructions[0x8f] = Cpu::XXX;
			 					  
		self.instructions[0x90] = Cpu::BCC;
		self.instructions[0x91] = Cpu::STA;
		self.instructions[0x92] = Cpu::XXX;
		self.instructions[0x93] = Cpu::XXX;
		self.instructions[0x94] = Cpu::STY;
		self.instructions[0x95] = Cpu::STA;
		self.instructions[0x96] = Cpu::STX;
		self.instructions[0x97] = Cpu::XXX;
		self.instructions[0x98] = Cpu::TYA;
		self.instructions[0x99] = Cpu::STA;
		self.instructions[0x9a] = Cpu::TXS;
		self.instructions[0x9b] = Cpu::XXX;
		self.instructions[0x9c] = Cpu::XXX;
		self.instructions[0x9d] = Cpu::STA;
		self.instructions[0x9e] = Cpu::XXX;
		self.instructions[0x9f] = Cpu::XXX;
			 					  
		self.instructions[0xa0] = Cpu::LDY;
		self.instructions[0xa1] = Cpu::LDA;
		self.instructions[0xa2] = Cpu::LDX;
		self.instructions[0xa3] = Cpu::XXX;
		self.instructions[0xa4] = Cpu::LDY;
		self.instructions[0xa5] = Cpu::LDA;
		self.instructions[0xa6] = Cpu::LDX;
		self.instructions[0xa7] = Cpu::XXX;
		self.instructions[0xa8] = Cpu::TAY;
		self.instructions[0xa9] = Cpu::LDA;
		self.instructions[0xaa] = Cpu::TAX;
		self.instructions[0xab] = Cpu::XXX;
		self.instructions[0xac] = Cpu::LDY;
		self.instructions[0xad] = Cpu::LDA;
		self.instructions[0xae] = Cpu::LDX;
		self.instructions[0xaf] = Cpu::XXX;
			 					  
		self.instructions[0xb0] = Cpu::BCS;
		self.instructions[0xb1] = Cpu::LDA;
		self.instructions[0xb2] = Cpu::XXX;
		self.instructions[0xb3] = Cpu::XXX;
		self.instructions[0xb4] = Cpu::LDY;
		self.instructions[0xb5] = Cpu::LDA;
		self.instructions[0xb6] = Cpu::LDX;
		self.instructions[0xb7] = Cpu::XXX;
		self.instructions[0xb8] = Cpu::CLV;
		self.instructions[0xb9] = Cpu::LDA;
		self.instructions[0xba] = Cpu::TSX;
		self.instructions[0xbb] = Cpu::XXX;
		self.instructions[0xbc] = Cpu::LDY;
		self.instructions[0xbd] = Cpu::LDA;
		self.instructions[0xbe] = Cpu::LDX;
		self.instructions[0xbf] = Cpu::XXX;
			 					  
		self.instructions[0xc0] = Cpu::CPY;
		self.instructions[0xc1] = Cpu::CMP;
		self.instructions[0xc2] = Cpu::XXX;
		self.instructions[0xc3] = Cpu::XXX;
		self.instructions[0xc4] = Cpu::CPY;
		self.instructions[0xc5] = Cpu::CMP;
		self.instructions[0xc6] = Cpu::DEC;
		self.instructions[0xc7] = Cpu::XXX;
		self.instructions[0xc8] = Cpu::INY;
		self.instructions[0xc9] = Cpu::CMP;
		self.instructions[0xca] = Cpu::DEX;
		self.instructions[0xcb] = Cpu::XXX;
		self.instructions[0xcc] = Cpu::CPY;
		self.instructions[0xcd] = Cpu::CMP;
		self.instructions[0xce] = Cpu::DEC;
		self.instructions[0xcf] = Cpu::XXX;
			 					
		self.instructions[0xd0] = Cpu::BNE;
		self.instructions[0xd1] = Cpu::CMP;
		self.instructions[0xd2] = Cpu::XXX;
		self.instructions[0xd3] = Cpu::XXX;
		self.instructions[0xd4] = Cpu::XXX;
		self.instructions[0xd5] = Cpu::CMP;
		self.instructions[0xd6] = Cpu::DEC;
		self.instructions[0xd7] = Cpu::XXX;
		self.instructions[0xd8] = Cpu::CLD;
		self.instructions[0xd9] = Cpu::CMP;
		self.instructions[0xda] = Cpu::XXX;
		self.instructions[0xdb] = Cpu::XXX;
		self.instructions[0xdc] = Cpu::XXX;
		self.instructions[0xdd] = Cpu::CMP;
		self.instructions[0xde] = Cpu::DEC;
		self.instructions[0xdf] = Cpu::XXX;
			 					 
		self.instructions[0xe0] = Cpu::CPX;
		self.instructions[0xe1] = Cpu::SBC;
		self.instructions[0xe2] = Cpu::XXX;
		self.instructions[0xe3] = Cpu::XXX;
		self.instructions[0xe4] = Cpu::CPX;
		self.instructions[0xe5] = Cpu::SBC;
		self.instructions[0xe6] = Cpu::INC;
		self.instructions[0xe7] = Cpu::XXX;
		self.instructions[0xe8] = Cpu::INX;
		self.instructions[0xe9] = Cpu::SBC;
		self.instructions[0xea] = Cpu::NOP;
		self.instructions[0xeb] = Cpu::XXX;
		self.instructions[0xec] = Cpu::CPX;
		self.instructions[0xed] = Cpu::SBC;
		self.instructions[0xee] = Cpu::INC;
		self.instructions[0xef] = Cpu::XXX;
			 					  
		self.instructions[0xf0] = Cpu::BEQ;
		self.instructions[0xf1] = Cpu::SBC;
		self.instructions[0xf2] = Cpu::XXX;
		self.instructions[0xf3] = Cpu::XXX;
		self.instructions[0xf4] = Cpu::XXX;
		self.instructions[0xf5] = Cpu::SBC;
		self.instructions[0xf6] = Cpu::INC;
		self.instructions[0xf7] = Cpu::XXX;
		self.instructions[0xf8] = Cpu::SED;
		self.instructions[0xf9] = Cpu::SBC;
		self.instructions[0xfa] = Cpu::XXX;
		self.instructions[0xfb] = Cpu::XXX;
		self.instructions[0xfc] = Cpu::XXX;
		self.instructions[0xfd] = Cpu::SBC;
		self.instructions[0xfe] = Cpu::INC;
		self.instructions[0xff] = Cpu::XXX;


		//=========================================================================================================================
		//Adressing modes
		//=========================================================================================================================

		self.addressing_modes[0x00] = Cpu::IMP;
		self.addressing_modes[0x01] = Cpu::IZX;
		self.addressing_modes[0x02] = Cpu::XXX;
		self.addressing_modes[0x03] = Cpu::XXX;
		self.addressing_modes[0x04] = Cpu::XXX;
		self.addressing_modes[0x05] = Cpu::ZP0;
		self.addressing_modes[0x06] = Cpu::ZP0;
		self.addressing_modes[0x07] = Cpu::XXX;
		self.addressing_modes[0x08] = Cpu::IMP;
		self.addressing_modes[0x09] = Cpu::IMM;
		self.addressing_modes[0x0a] = Cpu::ZP0;
		self.addressing_modes[0x0b] = Cpu::XXX;
		self.addressing_modes[0x0c] = Cpu::XXX;
		self.addressing_modes[0x0d] = Cpu::ABS;
		self.addressing_modes[0x0e] = Cpu::ABS;
		self.addressing_modes[0x0f] = Cpu::XXX;
			  
		self.addressing_modes[0x10] = Cpu::REL;
		self.addressing_modes[0x11] = Cpu::IZY;
		self.addressing_modes[0x12] = Cpu::XXX;
		self.addressing_modes[0x13] = Cpu::XXX;
		self.addressing_modes[0x14] = Cpu::XXX;
		self.addressing_modes[0x15] = Cpu::ZPX;
		self.addressing_modes[0x16] = Cpu::ZPX;
		self.addressing_modes[0x17] = Cpu::XXX;
		self.addressing_modes[0x18] = Cpu::IMP;
		self.addressing_modes[0x19] = Cpu::ABY;
		self.addressing_modes[0x1a] = Cpu::XXX;
		self.addressing_modes[0x1b] = Cpu::XXX;
		self.addressing_modes[0x1c] = Cpu::XXX;
		self.addressing_modes[0x1d] = Cpu::ABX;
		self.addressing_modes[0x1e] = Cpu::ABX;
		self.addressing_modes[0x1f] = Cpu::XXX;
			  
		self.addressing_modes[0x20] = Cpu::ABS;
		self.addressing_modes[0x21] = Cpu::IZX;
		self.addressing_modes[0x22] = Cpu::XXX;
		self.addressing_modes[0x23] = Cpu::XXX;
		self.addressing_modes[0x24] = Cpu::ZP0;
		self.addressing_modes[0x25] = Cpu::ZP0;
		self.addressing_modes[0x26] = Cpu::ZP0;
		self.addressing_modes[0x27] = Cpu::XXX;
		self.addressing_modes[0x28] = Cpu::IMP;
		self.addressing_modes[0x29] = Cpu::IMM;
		self.addressing_modes[0x2a] = Cpu::ZP0;
		self.addressing_modes[0x2b] = Cpu::XXX;
		self.addressing_modes[0x2c] = Cpu::ABS;
		self.addressing_modes[0x2d] = Cpu::ABS;
		self.addressing_modes[0x2e] = Cpu::ABS;
		self.addressing_modes[0x2f] = Cpu::XXX;
			  
		self.addressing_modes[0x30] = Cpu::REL;
		self.addressing_modes[0x31] = Cpu::IZY;
		self.addressing_modes[0x32] = Cpu::XXX;
		self.addressing_modes[0x33] = Cpu::XXX;
		self.addressing_modes[0x34] = Cpu::XXX;
		self.addressing_modes[0x35] = Cpu::ZPX;
		self.addressing_modes[0x36] = Cpu::ZPX;
		self.addressing_modes[0x37] = Cpu::XXX;
		self.addressing_modes[0x38] = Cpu::IMP;
		self.addressing_modes[0x39] = Cpu::ABY;
		self.addressing_modes[0x3a] = Cpu::XXX;
		self.addressing_modes[0x3b] = Cpu::XXX;
		self.addressing_modes[0x3c] = Cpu::XXX;
		self.addressing_modes[0x3d] = Cpu::ABX;
		self.addressing_modes[0x3e] = Cpu::ABX;
		self.addressing_modes[0x3f] = Cpu::XXX;
			  
		self.addressing_modes[0x40] = Cpu::IMP;
		self.addressing_modes[0x41] = Cpu::IZX;
		self.addressing_modes[0x42] = Cpu::XXX;
		self.addressing_modes[0x43] = Cpu::XXX;
		self.addressing_modes[0x44] = Cpu::XXX;
		self.addressing_modes[0x45] = Cpu::ZP0;
		self.addressing_modes[0x46] = Cpu::ZP0;
		self.addressing_modes[0x47] = Cpu::XXX;
		self.addressing_modes[0x48] = Cpu::IMP;
		self.addressing_modes[0x49] = Cpu::IMM;
		self.addressing_modes[0x4a] = Cpu::ZP0;
		self.addressing_modes[0x4b] = Cpu::XXX;
		self.addressing_modes[0x4c] = Cpu::ABS;
		self.addressing_modes[0x4d] = Cpu::ABS;
		self.addressing_modes[0x4e] = Cpu::ABS;
		self.addressing_modes[0x4f] = Cpu::XXX;
			 		 
		self.addressing_modes[0x50] = Cpu::REL;
		self.addressing_modes[0x51] = Cpu::IZY;
		self.addressing_modes[0x52] = Cpu::XXX;
		self.addressing_modes[0x53] = Cpu::XXX;
		self.addressing_modes[0x54] = Cpu::XXX;
		self.addressing_modes[0x55] = Cpu::IZX;
		self.addressing_modes[0x56] = Cpu::IZX;
		self.addressing_modes[0x57] = Cpu::XXX;
		self.addressing_modes[0x58] = Cpu::IMP;
		self.addressing_modes[0x59] = Cpu::ABY;
		self.addressing_modes[0x5a] = Cpu::XXX;
		self.addressing_modes[0x5b] = Cpu::XXX;
		self.addressing_modes[0x5c] = Cpu::XXX;
		self.addressing_modes[0x5d] = Cpu::ABX;
		self.addressing_modes[0x5e] = Cpu::ABX;
		self.addressing_modes[0x5f] = Cpu::XXX;
			  
		self.addressing_modes[0x60] = Cpu::IMP;
		self.addressing_modes[0x61] = Cpu::IZX;
		self.addressing_modes[0x62] = Cpu::XXX;
		self.addressing_modes[0x63] = Cpu::XXX;
		self.addressing_modes[0x64] = Cpu::XXX;
		self.addressing_modes[0x65] = Cpu::ZP0;
		self.addressing_modes[0x66] = Cpu::ZP0;
		self.addressing_modes[0x67] = Cpu::XXX;
		self.addressing_modes[0x68] = Cpu::IMP;
		self.addressing_modes[0x69] = Cpu::IMM;
		self.addressing_modes[0x6a] = Cpu::ZP0;
		self.addressing_modes[0x6b] = Cpu::XXX;
		self.addressing_modes[0x6c] = Cpu::ABS;
		self.addressing_modes[0x6d] = Cpu::ABS;
		self.addressing_modes[0x6e] = Cpu::ABS;
		self.addressing_modes[0x6f] = Cpu::XXX;
					 
		self.addressing_modes[0x70] = Cpu::REL;
		self.addressing_modes[0x71] = Cpu::IZY;
		self.addressing_modes[0x72] = Cpu::XXX;
		self.addressing_modes[0x73] = Cpu::XXX;
		self.addressing_modes[0x74] = Cpu::XXX;
		self.addressing_modes[0x75] = Cpu::ZPX;
		self.addressing_modes[0x76] = Cpu::ZPX;
		self.addressing_modes[0x77] = Cpu::XXX;
		self.addressing_modes[0x78] = Cpu::IMP;
		self.addressing_modes[0x79] = Cpu::ABY;
		self.addressing_modes[0x7a] = Cpu::XXX;
		self.addressing_modes[0x7b] = Cpu::XXX;
		self.addressing_modes[0x7c] = Cpu::XXX;
		self.addressing_modes[0x7d] = Cpu::ABX;
		self.addressing_modes[0x7e] = Cpu::ABX;
		self.addressing_modes[0x7f] = Cpu::XXX;
			 		  
		self.addressing_modes[0x80] = Cpu::XXX;
		self.addressing_modes[0x81] = Cpu::IZX;
		self.addressing_modes[0x82] = Cpu::XXX;
		self.addressing_modes[0x83] = Cpu::XXX;
		self.addressing_modes[0x84] = Cpu::ZP0;
		self.addressing_modes[0x85] = Cpu::ZP0;
		self.addressing_modes[0x86] = Cpu::ZP0;
		self.addressing_modes[0x87] = Cpu::XXX;
		self.addressing_modes[0x88] = Cpu::IMP;
		self.addressing_modes[0x89] = Cpu::XXX;
		self.addressing_modes[0x8a] = Cpu::IMP;
		self.addressing_modes[0x8b] = Cpu::XXX;
		self.addressing_modes[0x8c] = Cpu::ABS;
		self.addressing_modes[0x8d] = Cpu::ABS;
		self.addressing_modes[0x8e] = Cpu::ABS;
		self.addressing_modes[0x8f] = Cpu::XXX;
				  
		self.addressing_modes[0x90] = Cpu::REL;
		self.addressing_modes[0x91] = Cpu::IZY;
		self.addressing_modes[0x92] = Cpu::XXX;
		self.addressing_modes[0x93] = Cpu::XXX;
		self.addressing_modes[0x94] = Cpu::ZPX;
		self.addressing_modes[0x95] = Cpu::ZPX;
		self.addressing_modes[0x96] = Cpu::ZPY;
		self.addressing_modes[0x97] = Cpu::XXX;
		self.addressing_modes[0x98] = Cpu::IMP;
		self.addressing_modes[0x99] = Cpu::ABY;
		self.addressing_modes[0x9a] = Cpu::IMP;
		self.addressing_modes[0x9b] = Cpu::XXX;
		self.addressing_modes[0x9c] = Cpu::XXX;
		self.addressing_modes[0x9d] = Cpu::ABX;
		self.addressing_modes[0x9e] = Cpu::XXX;
		self.addressing_modes[0x9f] = Cpu::XXX;
			 	  
		self.addressing_modes[0xa0] = Cpu::IMM;
		self.addressing_modes[0xa1] = Cpu::IZX;
		self.addressing_modes[0xa2] = Cpu::IMM;
		self.addressing_modes[0xa3] = Cpu::XXX;
		self.addressing_modes[0xa4] = Cpu::ZP0;
		self.addressing_modes[0xa5] = Cpu::ZP0;
		self.addressing_modes[0xa6] = Cpu::ZP0;
		self.addressing_modes[0xa7] = Cpu::XXX;
		self.addressing_modes[0xa8] = Cpu::IMP;
		self.addressing_modes[0xa9] = Cpu::IMM;
		self.addressing_modes[0xaa] = Cpu::IMP;
		self.addressing_modes[0xab] = Cpu::XXX;
		self.addressing_modes[0xac] = Cpu::ABS;
		self.addressing_modes[0xad] = Cpu::ABS;
		self.addressing_modes[0xae] = Cpu::ABS;
		self.addressing_modes[0xaf] = Cpu::XXX;
			 		  
		self.addressing_modes[0xb0] = Cpu::REL;
		self.addressing_modes[0xb1] = Cpu::IZY;
		self.addressing_modes[0xb2] = Cpu::XXX;
		self.addressing_modes[0xb3] = Cpu::XXX;
		self.addressing_modes[0xb4] = Cpu::ZPX;
		self.addressing_modes[0xb5] = Cpu::ZPX;
		self.addressing_modes[0xb6] = Cpu::ZPY;
		self.addressing_modes[0xb7] = Cpu::XXX;
		self.addressing_modes[0xb8] = Cpu::IMP;
		self.addressing_modes[0xb9] = Cpu::ABY;
		self.addressing_modes[0xba] = Cpu::IMP;
		self.addressing_modes[0xbb] = Cpu::XXX;
		self.addressing_modes[0xbc] = Cpu::ABX;
		self.addressing_modes[0xbd] = Cpu::ABX;
		self.addressing_modes[0xbe] = Cpu::ABY;
		self.addressing_modes[0xbf] = Cpu::XXX;
			 		  
		self.addressing_modes[0xc0] = Cpu::IMM;
		self.addressing_modes[0xc1] = Cpu::IZX;
		self.addressing_modes[0xc2] = Cpu::XXX;
		self.addressing_modes[0xc3] = Cpu::XXX;
		self.addressing_modes[0xc4] = Cpu::ZP0;
		self.addressing_modes[0xc5] = Cpu::ZP0;
		self.addressing_modes[0xc6] = Cpu::ZP0;
		self.addressing_modes[0xc7] = Cpu::XXX;
		self.addressing_modes[0xc8] = Cpu::IMP;
		self.addressing_modes[0xc9] = Cpu::IMM;
		self.addressing_modes[0xca] = Cpu::IMP;
		self.addressing_modes[0xcb] = Cpu::XXX;
		self.addressing_modes[0xcc] = Cpu::ABS;
		self.addressing_modes[0xcd] = Cpu::ABS;
		self.addressing_modes[0xce] = Cpu::ABS;
		self.addressing_modes[0xcf] = Cpu::XXX;
			 		
		self.addressing_modes[0xd0] = Cpu::REL;
		self.addressing_modes[0xd1] = Cpu::IZY;
		self.addressing_modes[0xd2] = Cpu::XXX;
		self.addressing_modes[0xd3] = Cpu::XXX;
		self.addressing_modes[0xd4] = Cpu::XXX;
		self.addressing_modes[0xd5] = Cpu::ZPX;
		self.addressing_modes[0xd6] = Cpu::ZPX;
		self.addressing_modes[0xd7] = Cpu::XXX;
		self.addressing_modes[0xd8] = Cpu::IMP;
		self.addressing_modes[0xd9] = Cpu::ABY;
		self.addressing_modes[0xda] = Cpu::XXX;
		self.addressing_modes[0xdb] = Cpu::XXX;
		self.addressing_modes[0xdc] = Cpu::XXX;
		self.addressing_modes[0xdd] = Cpu::ABY;
		self.addressing_modes[0xde] = Cpu::ABX;
		self.addressing_modes[0xdf] = Cpu::XXX;
			 		 
		self.addressing_modes[0xe0] = Cpu::IMM;
		self.addressing_modes[0xe1] = Cpu::IZX;
		self.addressing_modes[0xe2] = Cpu::XXX;
		self.addressing_modes[0xe3] = Cpu::XXX;
		self.addressing_modes[0xe4] = Cpu::ZP0;
		self.addressing_modes[0xe5] = Cpu::ZP0;
		self.addressing_modes[0xe6] = Cpu::ZP0;
		self.addressing_modes[0xe7] = Cpu::XXX;
		self.addressing_modes[0xe8] = Cpu::IMP;
		self.addressing_modes[0xe9] = Cpu::IMM;
		self.addressing_modes[0xea] = Cpu::IMP;
		self.addressing_modes[0xeb] = Cpu::XXX;
		self.addressing_modes[0xec] = Cpu::ABS;
		self.addressing_modes[0xed] = Cpu::ABS;
		self.addressing_modes[0xee] = Cpu::ABS;
		self.addressing_modes[0xef] = Cpu::XXX;
			 	  
		self.addressing_modes[0xf0] = Cpu::REL;
		self.addressing_modes[0xf1] = Cpu::IZY;
		self.addressing_modes[0xf2] = Cpu::XXX;
		self.addressing_modes[0xf3] = Cpu::XXX;
		self.addressing_modes[0xf4] = Cpu::XXX;
		self.addressing_modes[0xf5] = Cpu::ZPX;
		self.addressing_modes[0xf6] = Cpu::ZPX;
		self.addressing_modes[0xf7] = Cpu::XXX;
		self.addressing_modes[0xf8] = Cpu::IMP;
		self.addressing_modes[0xf9] = Cpu::ABY;
		self.addressing_modes[0xfa] = Cpu::XXX;
		self.addressing_modes[0xfb] = Cpu::XXX;
		self.addressing_modes[0xfc] = Cpu::XXX;
		self.addressing_modes[0xfd] = Cpu::ABX;
		self.addressing_modes[0xfe] = Cpu::ABX;
		self.addressing_modes[0xff] = Cpu::XXX;

		
		//=========================================================================================================================
		//mnemonics
		//=========================================================================================================================

		self.mnemonics[0x00] = "BRK";
		self.mnemonics[0x01] = "ORA";
		self.mnemonics[0x02] = "XXX";
		self.mnemonics[0x03] = "XXX";
		self.mnemonics[0x04] = "XXX";
		self.mnemonics[0x05] = "ORA";
		self.mnemonics[0x06] = "ASL";
		self.mnemonics[0x07] = "XXX";
		self.mnemonics[0x08] = "PHP";
		self.mnemonics[0x09] = "ORA";
		self.mnemonics[0x0a] = "ASL";
		self.mnemonics[0x0b] = "XXX";
		self.mnemonics[0x0c] = "XXX";
		self.mnemonics[0x0d] = "ORA";
		self.mnemonics[0x0e] = "ASL";
		self.mnemonics[0x0f] = "XXX";
		
		self.mnemonics[0x10] = "BPL";
		self.mnemonics[0x11] = "ORA";
		self.mnemonics[0x12] = "XXX";
		self.mnemonics[0x13] = "XXX";
		self.mnemonics[0x14] = "XXX";
		self.mnemonics[0x15] = "ORA";
		self.mnemonics[0x16] = "ASL";
		self.mnemonics[0x17] = "XXX";
		self.mnemonics[0x18] = "CLC";
		self.mnemonics[0x19] = "ORA";
		self.mnemonics[0x1a] = "XXX";
		self.mnemonics[0x1b] = "XXX";
		self.mnemonics[0x1c] = "XXX";
		self.mnemonics[0x1d] = "ORA";
		self.mnemonics[0x1e] = "ASL";
		self.mnemonics[0x1f] = "XXX";

		self.mnemonics[0x20] = "JSR";
		self.mnemonics[0x21] = "AND";
		self.mnemonics[0x22] = "XXX";
		self.mnemonics[0x23] = "XXX";
		self.mnemonics[0x24] = "BIT";
		self.mnemonics[0x25] = "AND";
		self.mnemonics[0x26] = "ROL";
		self.mnemonics[0x27] = "XXX";
		self.mnemonics[0x28] = "PLP";
		self.mnemonics[0x29] = "AND";
		self.mnemonics[0x2a] = "ROL";
		self.mnemonics[0x2b] = "XXX";
		self.mnemonics[0x2c] = "BIT";
		self.mnemonics[0x2d] = "AND";
		self.mnemonics[0x2e] = "ROL";
		self.mnemonics[0x2f] = "XXX";

		self.mnemonics[0x30] = "BMI";
		self.mnemonics[0x31] = "AND";
		self.mnemonics[0x32] = "XXX";
		self.mnemonics[0x33] = "XXX";
		self.mnemonics[0x34] = "XXX";
		self.mnemonics[0x35] = "AND";
		self.mnemonics[0x36] = "ROL";
		self.mnemonics[0x37] = "XXX";
		self.mnemonics[0x38] = "SEC";
		self.mnemonics[0x39] = "AND";
		self.mnemonics[0x3a] = "XXX";
		self.mnemonics[0x3b] = "XXX";
		self.mnemonics[0x3c] = "XXX";
		self.mnemonics[0x3d] = "AND";
		self.mnemonics[0x3e] = "ROL";
		self.mnemonics[0x3f] = "XXX";

		self.mnemonics[0x40] = "RTI";
		self.mnemonics[0x41] = "EOR";
		self.mnemonics[0x42] = "XXX";
		self.mnemonics[0x43] = "XXX";
		self.mnemonics[0x44] = "XXX";
		self.mnemonics[0x45] = "EOR";
		self.mnemonics[0x46] = "LSR";
		self.mnemonics[0x47] = "XXX";
		self.mnemonics[0x48] = "PHA";
		self.mnemonics[0x49] = "EOR";
		self.mnemonics[0x4a] = "LSR";
		self.mnemonics[0x4b] = "XXX";
		self.mnemonics[0x4c] = "JMP";
		self.mnemonics[0x4d] = "EOR";
		self.mnemonics[0x4e] = "LSR";
		self.mnemonics[0x4f] = "XXX";

		self.mnemonics[0x50] = "BVC";
		self.mnemonics[0x51] = "EOR";
		self.mnemonics[0x52] = "XXX";
		self.mnemonics[0x53] = "XXX";
		self.mnemonics[0x54] = "XXX";
		self.mnemonics[0x55] = "EOR";
		self.mnemonics[0x56] = "LSR";
		self.mnemonics[0x57] = "XXX";
		self.mnemonics[0x58] = "CLI";
		self.mnemonics[0x59] = "EOR";
		self.mnemonics[0x5a] = "XXX";
		self.mnemonics[0x5b] = "XXX";
		self.mnemonics[0x5c] = "XXX";
		self.mnemonics[0x5d] = "EOR";
		self.mnemonics[0x5e] = "LSR";
		self.mnemonics[0x5f] = "XXX";

		self.mnemonics[0x60] = "RTS";
		self.mnemonics[0x61] = "ADC";
		self.mnemonics[0x62] = "XXX";
		self.mnemonics[0x63] = "XXX";
		self.mnemonics[0x64] = "XXX";
		self.mnemonics[0x65] = "ADC";
		self.mnemonics[0x66] = "ROR";
		self.mnemonics[0x67] = "XXX";
		self.mnemonics[0x68] = "PLA";
		self.mnemonics[0x69] = "ADC";
		self.mnemonics[0x6a] = "ROR";
		self.mnemonics[0x6b] = "XXX";
		self.mnemonics[0x6c] = "JMP";
		self.mnemonics[0x6d] = "ADC";
		self.mnemonics[0x6e] = "ROR";
		self.mnemonics[0x6f] = "XXX";

		self.mnemonics[0x70] = "BVS";
		self.mnemonics[0x71] = "ADC";
		self.mnemonics[0x72] = "XXX";
		self.mnemonics[0x73] = "XXX";
		self.mnemonics[0x74] = "XXX";
		self.mnemonics[0x75] = "ADC";
		self.mnemonics[0x76] = "ROR";
		self.mnemonics[0x77] = "XXX";
		self.mnemonics[0x78] = "SEI";
		self.mnemonics[0x79] = "ADC";
		self.mnemonics[0x7a] = "XXX";
		self.mnemonics[0x7b] = "XXX";
		self.mnemonics[0x7c] = "XXX";
		self.mnemonics[0x7d] = "ADC";
		self.mnemonics[0x7e] = "ROR";
		self.mnemonics[0x7f] = "XXX";

		self.mnemonics[0x80] = "XXX";
		self.mnemonics[0x81] = "STA";
		self.mnemonics[0x82] = "XXX";
		self.mnemonics[0x83] = "XXX";
		self.mnemonics[0x84] = "STY";
		self.mnemonics[0x85] = "STA";
		self.mnemonics[0x86] = "STX";
		self.mnemonics[0x87] = "XXX";
		self.mnemonics[0x88] = "DEY";
		self.mnemonics[0x89] = "XXX";
		self.mnemonics[0x8a] = "TXA";
		self.mnemonics[0x8b] = "XXX";
		self.mnemonics[0x8c] = "STY";
		self.mnemonics[0x8d] = "STA";
		self.mnemonics[0x8e] = "STX";
		self.mnemonics[0x8f] = "XXX";

		self.mnemonics[0x90] = "BCC";
		self.mnemonics[0x91] = "STA";
		self.mnemonics[0x92] = "XXX";
		self.mnemonics[0x93] = "XXX";
		self.mnemonics[0x94] = "STY";
		self.mnemonics[0x95] = "STA";
		self.mnemonics[0x96] = "STX";
		self.mnemonics[0x97] = "XXX";
		self.mnemonics[0x98] = "TYA";
		self.mnemonics[0x99] = "STA";
		self.mnemonics[0x9a] = "TXS";
		self.mnemonics[0x9b] = "XXX";
		self.mnemonics[0x9c] = "XXX";
		self.mnemonics[0x9d] = "STA";
		self.mnemonics[0x9e] = "XXX";
		self.mnemonics[0x9f] = "XXX";

		self.mnemonics[0xa0] = "LDY";
		self.mnemonics[0xa1] = "LDA";
		self.mnemonics[0xa2] = "LDX";
		self.mnemonics[0xa3] = "XXX";
		self.mnemonics[0xa4] = "LDY";
		self.mnemonics[0xa5] = "LDA";
		self.mnemonics[0xa6] = "LDX";
		self.mnemonics[0xa7] = "XXX";
		self.mnemonics[0xa8] = "TAY";
		self.mnemonics[0xa9] = "LDA";
		self.mnemonics[0xaa] = "TAX";
		self.mnemonics[0xab] = "XXX";
		self.mnemonics[0xac] = "LDY";
		self.mnemonics[0xad] = "LDA";
		self.mnemonics[0xae] = "LDX";
		self.mnemonics[0xaf] = "XXX";

		self.mnemonics[0xb0] = "BCS";
		self.mnemonics[0xb1] = "LDA";
		self.mnemonics[0xb2] = "XXX";
		self.mnemonics[0xb3] = "XXX";
		self.mnemonics[0xb4] = "LDY";
		self.mnemonics[0xb5] = "LDA";
		self.mnemonics[0xb6] = "LDX";
		self.mnemonics[0xb7] = "XXX";
		self.mnemonics[0xb8] = "CLV";
		self.mnemonics[0xb9] = "LDA";
		self.mnemonics[0xba] = "TSX";
		self.mnemonics[0xbb] = "XXX";
		self.mnemonics[0xbc] = "LDY";
		self.mnemonics[0xbd] = "LDA";
		self.mnemonics[0xbe] = "LDX";
		self.mnemonics[0xbf] = "XXX";

		self.mnemonics[0xc0] = "CPY";
		self.mnemonics[0xc1] = "CMP";
		self.mnemonics[0xc2] = "XXX";
		self.mnemonics[0xc3] = "XXX";
		self.mnemonics[0xc4] = "CPY";
		self.mnemonics[0xc5] = "CMP";
		self.mnemonics[0xc6] = "DEC";
		self.mnemonics[0xc7] = "XXX";
		self.mnemonics[0xc8] = "INY";
		self.mnemonics[0xc9] = "CMP";
		self.mnemonics[0xca] = "DEX";
		self.mnemonics[0xcb] = "XXX";
		self.mnemonics[0xcc] = "CPY";
		self.mnemonics[0xcd] = "CMP";
		self.mnemonics[0xce] = "DEC";
		self.mnemonics[0xcf] = "XXX";

		self.mnemonics[0xd0] = "BNE";
		self.mnemonics[0xd1] = "CMP";
		self.mnemonics[0xd2] = "XXX";
		self.mnemonics[0xd3] = "XXX";
		self.mnemonics[0xd4] = "XXX";
		self.mnemonics[0xd5] = "CMP";
		self.mnemonics[0xd6] = "DEC";
		self.mnemonics[0xd7] = "XXX";
		self.mnemonics[0xd8] = "CLD";
		self.mnemonics[0xd9] = "CMP";
		self.mnemonics[0xda] = "XXX";
		self.mnemonics[0xdb] = "XXX";
		self.mnemonics[0xdc] = "XXX";
		self.mnemonics[0xdd] = "CMP";
		self.mnemonics[0xde] = "DEC";
		self.mnemonics[0xdf] = "XXX";

		self.mnemonics[0xe0] = "CPX";
		self.mnemonics[0xe1] = "SBC";
		self.mnemonics[0xe2] = "XXX";
		self.mnemonics[0xe3] = "XXX";
		self.mnemonics[0xe4] = "CPX";
		self.mnemonics[0xe5] = "SBC";
		self.mnemonics[0xe6] = "INC";
		self.mnemonics[0xe7] = "XXX";
		self.mnemonics[0xe8] = "INX";
		self.mnemonics[0xe9] = "SBC";
		self.mnemonics[0xea] = "NOP";
		self.mnemonics[0xeb] = "XXX";
		self.mnemonics[0xec] = "CPX";
		self.mnemonics[0xed] = "SBC";
		self.mnemonics[0xee] = "INC";
		self.mnemonics[0xef] = "XXX";

		self.mnemonics[0xf0] = "BEQ";
		self.mnemonics[0xf1] = "SBC";
		self.mnemonics[0xf2] = "XXX";
		self.mnemonics[0xf3] = "XXX";
		self.mnemonics[0xf4] = "XXX";
		self.mnemonics[0xf5] = "SBC";
		self.mnemonics[0xf6] = "INC";
		self.mnemonics[0xf7] = "XXX";
		self.mnemonics[0xf8] = "SED";
		self.mnemonics[0xf9] = "SBC";
		self.mnemonics[0xfa] = "XXX";
		self.mnemonics[0xfb] = "XXX";
		self.mnemonics[0xfc] = "XXX";
		self.mnemonics[0xfd] = "SBC";
		self.mnemonics[0xfe] = "INC";
		self.mnemonics[0xff] = "XXX";
	}
}
