use std::io;
use std::io::prelude::*;
use chrono::prelude::*;

fn main() {
	systems_check_up();

	let date = Utc::now();			
    println!("Welcome to monks. Today is {}", date.format("%Y-%m-%d"));

	let stdin = io::stdin();
	for l in stdin.lock().lines() {
		let cmd_to_execute = l.unwrap();	
		println!("command to execute: {}", cmd_to_execute);
	}
}

fn systems_check_up() {
	/*
		loop through the different systems and start them up
		checking whether they start up successfully.
		e.g.: brain_system.start_up()		
	*/
	let systems = [0];

	for s in systems {
		// e.g.: s.start_up()
	}
}

