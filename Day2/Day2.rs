use std::env;
use std::fs;


fn letter_to_val(letter: char) -> i32 {
	match letter {
		'A' => 1, // Rock
		'B' => 2, // Paper
		'C' => 3, // Scissors
		'X' => 1, // Rock
		'Y' => 2, // Paper
		'Z' => 3,  // Scissors		
                _ => panic!("Bad letter: {}", letter)
	}
}

fn calc_my_score(opponent_shape: i32, my_shape: i32) -> i32 {
	if my_shape == opponent_shape {
		return my_shape + 3;
	}
	else {
		match my_shape {
			1 => {
				match opponent_shape {
					2 => return my_shape,
					3 => return my_shape + 6,
					i32::MIN..=1_i32 | 4_i32..=i32::MAX => return 0
					}
			},
			2 => {
				match opponent_shape {
					1 => return my_shape + 6,
					3 => return my_shape,
					i32::MIN..=2_i32 | 4_i32..=i32::MAX => return 0
					}
			},
			3 => {
				match opponent_shape {
					1 => return my_shape,
					2 => return my_shape + 6,
					i32::MIN..= 0_i32 | 3_i32..=i32::MAX => return 0
				}
			},
			i32::MIN..=1_i32 | 4_i32..=i32::MAX => return 0
		}

	}
}

fn calc_my_score_part2(opponent_shape: char, my_result: char) -> i32 {
	
		match opponent_shape {
			'A' => { // ROCK
				match my_result {
					'X' => return 0 + 3, // lost, scissor
					'Y' => return 3 + 1, // draw, rock
					'Z' => return 6 + 2, // win, paper
					_ => panic!("Bad letter")
					}
			},
			'B' => { // PAPER
				match my_result {
					'X' => return 0 + 1, // lost, rock
					'Y' => return 3 + 2, // draw, paper
					'Z' => return 6 + 3, // win, scissor
					_ => panic!("Bad letter")
					}
			},
			'C' => { // SCISSOR
				match my_result {
					'X' => return 0 + 2, // lost, paper
					'Y' => return 3 + 3, // draw, scissor
					'Z' => return 6 + 1, // win, rock
					_ => panic!("Bad letter")
					}
			},
					_ => panic!("Bad letter")
		}

	
}

fn main() {
	println!("DAY 2");
	let args: Vec<String>= env::args().collect();
	let file_path = &args[1];

	let orig_contents = fs::read_to_string(file_path).expect("Couldn't open file");
	let rounds: Vec<&str> = orig_contents.split("\n").collect();
	let mut score_part_1 = 0;
	let mut score_part_2 = 0;
	
	for line in rounds{
		if line == "" {break;}
		
		// PART 1
		score_part_1 += calc_my_score(letter_to_val(line.chars().nth(0).unwrap()),letter_to_val(line.chars().nth(2).unwrap()));
		
		// PART 2
		score_part_2 += calc_my_score_part2(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
	}
	
	
	println!("PART 1: {score_part_1}");	
	println!("PART 2: {score_part_2}");
}


