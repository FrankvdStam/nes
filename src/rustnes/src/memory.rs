use std::cell::RefCell;
use crate::Bus;

pub struct Memory
{
	bus: RefCell<Bus>,
}

impl Memory
{
	pub fn new(bus: RefCell<Bus>) -> Self
	{
		Memory
		{
			bus: bus,
		}
	}
}