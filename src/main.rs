#[allow(dead_code)]
#[allow(non_snake_case)]

mod cpu;
mod bus;
mod memory;

use cpu::Cpu;
use bus::Bus;

use std::cell::RefCell;


fn main()
{

	let mut i:i32 = 1;
    let ref_i = &mut i;
	*ref_i = 1;
    let another_ref_i = &mut i;
	*another_ref_i = 1;

	//let mut cpu = Cpu::new();
	println!("Hello, world!");
}
