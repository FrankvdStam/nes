use crate::Cpu;
use crate::Bus;
use crate::Memory;

pub struct Nes
{
	cpu: Cpu,
	bus: RefCell<Bus>,
}

impl Nes
{
	pub fn new() -> Self
	{
		let bus = RefCell::new(Bus::new());
		let cpu = Cpu::new(bus);
		let memory = Memory::new(bus);

		let nes = Nes
		{
			cpu: cpu,
			bus: bus,
		}
	}
}