use std::env;
use std::fs;
use std::collections::HashSet;

fn item_to_priority(item:&char) -> u64
{
	let val = *item as u64;
	if val < 97 {
		return val - 38;
	}
	return val - 96;		
}

fn main() {
	println!("DAY 3");
	let args: Vec<String>= env::args().collect();
	let file_path = &args[1];

	let orig_contents = fs::read_to_string(file_path).expect("Couldn't open file");
	let contents: Vec<&str> = orig_contents.split("\n").collect();
	let mut part2 :u64 = 0;
/*	
	let mut part1 :u64 = 0;
	for line in contents{
		if line == "" {break;}
		// PART 1
		let halfway = line.chars().count()/2;
		let (first_half, sec_half) = line.split_at(halfway);
		let first_items : HashSet<char>= first_half.chars().collect();
		let sec_items : HashSet<char>= sec_half.chars().collect();
		
		part1 += item_to_priority(first_items.intersection(&sec_items).next().unwrap());
	}
	println!("PART1: {part1}");
*/

	// How do I initialize an empty HashSet? Can I?
	let mut first_items :HashSet<char> = HashSet::with_capacity(100);
	let mut sec_items :HashSet<char> = HashSet::with_capacity(100);
	for (index, line) in contents.iter().enumerate() {
		if index % 3 == 0 {
			first_items = line.chars().collect();
		}
		else if index % 3 == 1 {
			sec_items =line.chars().collect();
		}
		else {
			let third_items = line.chars().collect();
				
			let temp_set : HashSet<char> = first_items.intersection(&sec_items).copied().collect();
			part2 += item_to_priority(temp_set.intersection(&third_items).next().unwrap());
		}
	
	}
	println!("PART2: {part2}");	
}


