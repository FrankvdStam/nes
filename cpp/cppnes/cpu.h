#pragma once
#include <cstdint>
#include <string>

class cpu
{
public:
	
	
private:
	//I/O
	uint16_t address = 0;
	uint8_t data = 0;
	bool read = true;

	//Registers
	uint16_t program_counter = 0;
	uint8_t accumulator = 0;
	
	uint8_t index_register_y = 0;
	uint8_t index_register_x = 0;
	uint8_t stack_pointer = 0;
	
	//Processor status is actually an 8bit register, I decided to write them out to fields
	bool carry					  = false;
	bool zero					  = false;
	bool interupt_request_disable = false;
	bool decimal_mode			  = false;
	bool break_command			  = false;
	bool overflow				  = false;
	bool negative				  = false;

	//=========================================================================================================================
	//Addressing modes
	//Each addressing mode returns a 0 or 1 corresponding to wether or not they may require another cpu cycle to complete the
	//full instruction.
	//=========================================================================================================================
	uint8_t IMP();
	uint8_t IMM();
	uint8_t ZP0();
	uint8_t ZPX();
	uint8_t ZPY();
	uint8_t REL();
	uint8_t ABS();
	uint8_t ABX();
	uint8_t ABY();
	uint8_t IND();
	uint8_t IZX();
	uint8_t IZY();

	
	//=========================================================================================================================
	//Instructions
	//Each instruction returns 1 if it might need another cpu cycle to complete the full instruction
	//=========================================================================================================================

	//Add Memory to Accumulator with Carry
	uint8_t ADC();

	//"AND" Memory with Accumulator
	uint8_t AND();

	//Shift left One Bit(Memory or Accumulator)
	uint8_t ASL();

	//Branch on Carry Clear
	uint8_t BCC();

	//Branch on Carry Set
	uint8_t BCS();

	//Branch on Result Zero
	uint8_t BEQ();

	//Test Bits in Memory with Accumulator
	uint8_t BIT();

	//Branch on Result Minus
	uint8_t BMI();

	//Branch on Result not Zero
	uint8_t BNE();

	//Branch on Result Plus
	uint8_t BPL();

	//Force Break
	uint8_t BRK();

	//Branch on Overflow Clear
	uint8_t BVC();

	//Branch on Overflow Set
	uint8_t BVS();

	//Clear Carry Flag
	uint8_t CLC();

	//Clear Decimal Mode
	uint8_t CLD();

	//Clear Interrupt Disable Bit
	uint8_t CLI();

	//Clear Overflow Flag
	uint8_t CLV();

	//Compare Memory and Accumulator
	uint8_t CMP();

	//Compare Memory and Index X
	uint8_t CPX();

	//Compare Memory and Index Y
	uint8_t CPY();
	
	//Decrement Memory by One
	uint8_t DEC();


	//Decrement Index X by One
	uint8_t DEX();

	//Decrement Index Y by One
	uint8_t DEY();

	//"Exclusive-or" Memory with Accumulator
	uint8_t EOR();

	//Increment Memory by One
	uint8_t INC();

	//Increment Index X by One
	uint8_t INX();

	//Increment Index Y by One
	uint8_t INY();

	//Jump to New Location
	uint8_t JMP();

	//Jump to New Location Saving Return Address
	uint8_t JSR();

	//Load Accumulator with Memory
	uint8_t LDA();

	//Load Index X with Memory
	uint8_t LDX();

	//Load Index Y with Memory
	uint8_t LDY();

	//Shift One Bit Right(Memory or Accumulator)
	uint8_t LSR();

	//No Operation
	uint8_t NOP();

	//"OR" Memory with Accumulator
	uint8_t ORA();
	

	//Push Accumulator on Stack
	uint8_t PHA();

	//Push Processor Statuson Stack
	uint8_t PHP();

	//Pull Accumulator from Stack
	uint8_t PLA();

	//Pull Processor Status from Stack
	uint8_t PLP();

	//Rotate One Bit Left(Memory or Accumulator)
	uint8_t ROL();

	//Rotate One Bit Right(Memory or Accumulator)
	uint8_t ROR();

	//Return from Interrupt
	uint8_t RTI();

	//Return from Subroutine
	uint8_t RTS();

	//Subtract Memory from Accumulator with Borrow
	uint8_t SBC();

	//Set Carry Flag
	uint8_t SEC();

	//Set Decimal Mode
	uint8_t SED();

	uint8_t SEI();
	//Set Interrupt Disable Status

	//Store Accumulator in Memory
	uint8_t STA();

	//Store Index X in Memory
	uint8_t STX();

	//Store Index Y in Memory
	uint8_t STY();

	//Transfer Accumulator to Index X
	uint8_t TAX();

	//Transfer Accumulator to Index Y
	uint8_t TAY();

	//Transfer Stack Pointer to Index
	uint8_t TSX();

	//Transfer Index X to Accumulator
	uint8_t TXA();

	//Transfer Index X to Stack Pointer
	uint8_t TXS();

	//Transfer Index Y to Accumulator
	uint8_t TYA();



	//"illegal" instruction
	uint8_t XXX();
	
	struct instruction
	{
		std::string name;
		//void (*addressing_mode_function)(uint8_t);
		//void (*instruction_function)(uint8_t);

		uint8_t(cpu::*addressing_mode_function)(void);
		uint8_t(cpu::*instruction_function)(void);
		uint8_t cycles;
	};


	void init_instructions()
	{
		instruction instructions[] = {
			{"BRK", &cpu::addressing_mode_accumulator, &cpu::addressing_mode_accumulator, 7}
		};

		
		lookup =
		{
			{ "BRK", &cpu::BRK, &cpu::IMM, 7 },{ "ORA", &cpu::ORA, &cpu::IZX, 6 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 3 },{ "ORA", &cpu::ORA, &cpu::ZP0, 3 },{ "ASL", &cpu::ASL, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "PHP", &cpu::PHP, &cpu::IMP, 3 },{ "ORA", &cpu::ORA, &cpu::IMM, 2 },{ "ASL", &cpu::ASL, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "ORA", &cpu::ORA, &cpu::ABS, 4 },{ "ASL", &cpu::ASL, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BPL", &cpu::BPL, &cpu::REL, 2 },{ "ORA", &cpu::ORA, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "ORA", &cpu::ORA, &cpu::ZPX, 4 },{ "ASL", &cpu::ASL, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "CLC", &cpu::CLC, &cpu::IMP, 2 },{ "ORA", &cpu::ORA, &cpu::ABY, 4 },{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "ORA", &cpu::ORA, &cpu::ABX, 4 },{ "ASL", &cpu::ASL, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
			{ "JSR", &cpu::JSR, &cpu::ABS, 6 },{ "AND", &cpu::AND, &cpu::IZX, 6 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "BIT", &cpu::BIT, &cpu::ZP0, 3 },{ "AND", &cpu::AND, &cpu::ZP0, 3 },{ "ROL", &cpu::ROL, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "PLP", &cpu::PLP, &cpu::IMP, 4 },{ "AND", &cpu::AND, &cpu::IMM, 2 },{ "ROL", &cpu::ROL, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "BIT", &cpu::BIT, &cpu::ABS, 4 },{ "AND", &cpu::AND, &cpu::ABS, 4 },{ "ROL", &cpu::ROL, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BMI", &cpu::BMI, &cpu::REL, 2 },{ "AND", &cpu::AND, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "AND", &cpu::AND, &cpu::ZPX, 4 },{ "ROL", &cpu::ROL, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "SEC", &cpu::SEC, &cpu::IMP, 2 },{ "AND", &cpu::AND, &cpu::ABY, 4 },{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "AND", &cpu::AND, &cpu::ABX, 4 },{ "ROL", &cpu::ROL, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
			{ "RTI", &cpu::RTI, &cpu::IMP, 6 },{ "EOR", &cpu::EOR, &cpu::IZX, 6 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 3 },{ "EOR", &cpu::EOR, &cpu::ZP0, 3 },{ "LSR", &cpu::LSR, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "PHA", &cpu::PHA, &cpu::IMP, 3 },{ "EOR", &cpu::EOR, &cpu::IMM, 2 },{ "LSR", &cpu::LSR, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "JMP", &cpu::JMP, &cpu::ABS, 3 },{ "EOR", &cpu::EOR, &cpu::ABS, 4 },{ "LSR", &cpu::LSR, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BVC", &cpu::BVC, &cpu::REL, 2 },{ "EOR", &cpu::EOR, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "EOR", &cpu::EOR, &cpu::ZPX, 4 },{ "LSR", &cpu::LSR, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "CLI", &cpu::CLI, &cpu::IMP, 2 },{ "EOR", &cpu::EOR, &cpu::ABY, 4 },{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "EOR", &cpu::EOR, &cpu::ABX, 4 },{ "LSR", &cpu::LSR, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
			{ "RTS", &cpu::RTS, &cpu::IMP, 6 },{ "ADC", &cpu::ADC, &cpu::IZX, 6 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 3 },{ "ADC", &cpu::ADC, &cpu::ZP0, 3 },{ "ROR", &cpu::ROR, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "PLA", &cpu::PLA, &cpu::IMP, 4 },{ "ADC", &cpu::ADC, &cpu::IMM, 2 },{ "ROR", &cpu::ROR, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "JMP", &cpu::JMP, &cpu::IND, 5 },{ "ADC", &cpu::ADC, &cpu::ABS, 4 },{ "ROR", &cpu::ROR, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BVS", &cpu::BVS, &cpu::REL, 2 },{ "ADC", &cpu::ADC, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "ADC", &cpu::ADC, &cpu::ZPX, 4 },{ "ROR", &cpu::ROR, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "SEI", &cpu::SEI, &cpu::IMP, 2 },{ "ADC", &cpu::ADC, &cpu::ABY, 4 },{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "ADC", &cpu::ADC, &cpu::ABX, 4 },{ "ROR", &cpu::ROR, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
			{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "STA", &cpu::STA, &cpu::IZX, 6 },{ "???", cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "STY", &cpu::STY, &cpu::ZP0, 3 },{ "STA", &cpu::STA, &cpu::ZP0, 3 },{ "STX", &cpu::STX, &cpu::ZP0, 3 },{ "???", &cpu::XXX, &cpu::IMP, 3 },{ "DEY", &cpu::DEY, &cpu::IMP, 2 },{ "???", &cpu::NOP, &cpu::IMP, 2 },{ "TXA", &cpu::TXA, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "STY", &cpu::STY, &cpu::ABS, 4 },{ "STA", &cpu::STA, &cpu::ABS, 4 },{ "STX", &cpu::STX, &cpu::ABS, 4 },{ "???", &cpu::XXX, &cpu::IMP, 4 },
			{ "BCC", &cpu::BCC, &cpu::REL, 2 },{ "STA", &cpu::STA, &cpu::IZY, 6 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "STY", &cpu::STY, &cpu::ZPX, 4 },{ "STA", &cpu::STA, &cpu::ZPX, 4 },{ "STX", &cpu::STX, &cpu::ZPY, 4 },{ "???", &cpu::XXX, &cpu::IMP, 4 },{ "TYA", &cpu::TYA, &cpu::IMP, 2 },{ "STA", &cpu::STA, &cpu::ABY, 5 },{ "TXS", &cpu::TXS, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "???", &cpu::NOP, &cpu::IMP, 5 },{ "STA", &cpu::STA, &cpu::ABX, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },
			{ "LDY", &cpu::LDY, &cpu::IMM, 2 },{ "LDA", &cpu::LDA, &cpu::IZX, 6 },{ "LDX", cpu::LDX, &cpu::IMM, 2 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "LDY", &cpu::LDY, &cpu::ZP0, 3 },{ "LDA", &cpu::LDA, &cpu::ZP0, 3 },{ "LDX", &cpu::LDX, &cpu::ZP0, 3 },{ "???", &cpu::XXX, &cpu::IMP, 3 },{ "TAY", &cpu::TAY, &cpu::IMP, 2 },{ "LDA", &cpu::LDA, &cpu::IMM, 2 },{ "TAX", &cpu::TAX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "LDY", &cpu::LDY, &cpu::ABS, 4 },{ "LDA", &cpu::LDA, &cpu::ABS, 4 },{ "LDX", &cpu::LDX, &cpu::ABS, 4 },{ "???", &cpu::XXX, &cpu::IMP, 4 },
			{ "BCS", &cpu::BCS, &cpu::REL, 2 },{ "LDA", &cpu::LDA, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "LDY", &cpu::LDY, &cpu::ZPX, 4 },{ "LDA", &cpu::LDA, &cpu::ZPX, 4 },{ "LDX", &cpu::LDX, &cpu::ZPY, 4 },{ "???", &cpu::XXX, &cpu::IMP, 4 },{ "CLV", &cpu::CLV, &cpu::IMP, 2 },{ "LDA", &cpu::LDA, &cpu::ABY, 4 },{ "TSX", &cpu::TSX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 4 },{ "LDY", &cpu::LDY, &cpu::ABX, 4 },{ "LDA", &cpu::LDA, &cpu::ABX, 4 },{ "LDX", &cpu::LDX, &cpu::ABY, 4 },{ "???", &cpu::XXX, &cpu::IMP, 4 },
			{ "CPY", &cpu::CPY, &cpu::IMM, 2 },{ "CMP", &cpu::CMP, &cpu::IZX, 6 },{ "???", cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "CPY", &cpu::CPY, &cpu::ZP0, 3 },{ "CMP", &cpu::CMP, &cpu::ZP0, 3 },{ "DEC", &cpu::DEC, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "INY", &cpu::INY, &cpu::IMP, 2 },{ "CMP", &cpu::CMP, &cpu::IMM, 2 },{ "DEX", &cpu::DEX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 2 },{ "CPY", &cpu::CPY, &cpu::ABS, 4 },{ "CMP", &cpu::CMP, &cpu::ABS, 4 },{ "DEC", &cpu::DEC, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BNE", &cpu::BNE, &cpu::REL, 2 },{ "CMP", &cpu::CMP, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "CMP", &cpu::CMP, &cpu::ZPX, 4 },{ "DEC", &cpu::DEC, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "CLD", &cpu::CLD, &cpu::IMP, 2 },{ "CMP", &cpu::CMP, &cpu::ABY, 4 },{ "NOP", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "CMP", &cpu::CMP, &cpu::ABX, 4 },{ "DEC", &cpu::DEC, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
			{ "CPX", &cpu::CPX, &cpu::IMM, 2 },{ "SBC", &cpu::SBC, &cpu::IZX, 6 },{ "???", cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "CPX", &cpu::CPX, &cpu::ZP0, 3 },{ "SBC", &cpu::SBC, &cpu::ZP0, 3 },{ "INC", &cpu::INC, &cpu::ZP0, 5 },{ "???", &cpu::XXX, &cpu::IMP, 5 },{ "INX", &cpu::INX, &cpu::IMP, 2 },{ "SBC", &cpu::SBC, &cpu::IMM, 2 },{ "NOP", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::SBC, &cpu::IMP, 2 },{ "CPX", &cpu::CPX, &cpu::ABS, 4 },{ "SBC", &cpu::SBC, &cpu::ABS, 4 },{ "INC", &cpu::INC, &cpu::ABS, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },
			{ "BEQ", &cpu::BEQ, &cpu::REL, 2 },{ "SBC", &cpu::SBC, &cpu::IZY, 5 },{ "???", cpu::XXX, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 8 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "SBC", &cpu::SBC, &cpu::ZPX, 4 },{ "INC", &cpu::INC, &cpu::ZPX, 6 },{ "???", &cpu::XXX, &cpu::IMP, 6 },{ "SED", &cpu::SED, &cpu::IMP, 2 },{ "SBC", &cpu::SBC, &cpu::ABY, 4 },{ "NOP", &cpu::NOP, &cpu::IMP, 2 },{ "???", &cpu::XXX, &cpu::IMP, 7 },{ "???", &cpu::NOP, &cpu::IMP, 4 },{ "SBC", &cpu::SBC, &cpu::ABX, 4 },{ "INC", &cpu::INC, &cpu::ABX, 7 },{ "???", &cpu::XXX, &cpu::IMP, 7 },
		};
	}
};

