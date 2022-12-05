use std::env;
use std::fs;

fn main() {
	println!("DAY 1");
	let args: Vec<String>= env::args().collect();
	let file_path = &args[1];

	let orig_contents = fs::read_to_string(file_path).expect("Couldn't open file");
	let contents: Vec<&str> = orig_contents.split("\n").collect();
	
	//let mut elf_maxes : Vec<usize> = 
		
	let mut max: Vec<i32> = vec![0,0,0];
	let mut elf_max: i32 = 0;
	let mut count = 0;
	
	for line in contents{
		if line != "" {
			elf_max += line.parse::<i32>().unwrap();
			//println!("{count} elf_max = {elf_max}");
		}
		else if elf_max > *max.iter().min().unwrap() {
			count +=1;
			//println!("{count}, {max}, {elf_max}");
			//max[max.iter.min(). = elf_max;
			let index = max.iter().position(|&r| r == *max.iter().min().unwrap()).unwrap();
			max[index] = elf_max;
			elf_max = 0;
		}
		else {
			elf_max = 0;
		}
	}
	
	let total : i32= max.iter().sum();
	println!("Total max calories = {total}");	
}

fn part1()
{
	let mut max: usize = 0;
	let mut elf_max: usize = 0;
	let mut count = 0;
	/*
	for line in contents{
		if line != "" {
			elf_max += line.parse::<usize>().unwrap();
			println!("{count} elf_max = {elf_max}");
		}
		else if elf_max > max {
			count +=1;
			println!("{count}, {max}, {elf_max}");
			max = elf_max;
			elf_max = 0;
		}
		else {
			elf_max = 0;
		}
	}*/
	
	println!("Max calories = {max}");

}
