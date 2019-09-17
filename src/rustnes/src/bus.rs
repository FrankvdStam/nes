pub struct Bus
{
	address: u16,
	data: u8,
}

impl Bus
{
	pub fn new() -> Self
	{
		Bus
		{
			address: 0,
			data: 0,
		}
	}

	pub fn read(&self) -> u8
	{
		return self.data;
	}
	
	pub fn write(&mut self, data: u8)
	{
		self.data = data;
	}

	pub fn set_address(&mut self, address: u16)
	{
		self.address = address;
	}
}