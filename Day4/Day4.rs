use std::env;
use std::fs;



fn main() {
	println!("DAY 4");
	let args: Vec<String>= env::args().collect();
	let file_path = &args[1];

	let orig_contents = fs::read_to_string(file_path).expect("Couldn't open file");
	let contents: Vec<&str> = orig_contents.split("\n").collect();
	let mut part1 = 0;
	let mut part2 = 0;
	for line in contents{
		if line == "" {continue;}
		let pair : Vec<&str>= line.split(",").collect();
		let first : Vec<i32> = pair[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
		let sec : Vec<i32> = pair[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
		
		// PART 1
		if first[0] >= sec[0] && first[1] <= sec[1]
		{
			part1 += 1;
		}
		else if sec[0] >= first[0] && sec[1] <= first[1]
		{
			part1 += 1;
		}
		
		// PART 2
		if first[0] >= sec[0] && first[0] <= sec[1]
		{
			part2 += 1;		
		}
		else if first[1] >= sec[0] && first[1] <= sec[1]
		{
			part2 += 1;
		}
		else if sec[0] >= first[0] && sec[0] <= first[1] 
		{
			part2 += 1;
		}
		else if sec[1] >= first[0] && sec[1] <= first[1]
		{
			part2 += 1;
		}
	}
	println!("PART1: {part1}");
	println!("PART2: {part2}");

	
}


