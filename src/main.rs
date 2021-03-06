#![windows_subsystem = "windows"]

#[macro_use]
extern crate serenity;
extern crate rand;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

use rand::Rng;

struct Handler;

impl EventHandler for Handler {}

#[cfg(feature = "external_token")]
fn log_in() -> Client {
	use std::fs::File;
	use std::io::Read;

	let mut token_file = File::open("./bot.token").expect("bot.token not found!");
    let mut token_string = String::new();

    token_file.read_to_string(&mut token_string).expect("Failed to read token from bot.token.");

	Client::new(&token_string[..], Handler).expect("Error creating client")
}

#[cfg(not(feature = "external_token"))]
fn log_in() -> Client {
	Client::new(include_str!("../bot.token"), Handler).expect("Error creating client")
}

fn main() {
    //Log in
    let mut client = log_in();

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("$"))
		.cmd("roll", roll_command)
		.cmd("r", roll_command)
		.cmd("fudge", roll_fudge_command)
		.cmd("f", roll_fudge_command));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(roll_command(_context, message) {
	let response = roll(&message.content);
	message.reply(&response[..]).expect("failed to send message");
});

command!(roll_fudge_command(_context, message) {
	let response = roll_fudge(&message.content);
	message.reply(&response[..]).expect("failed to send message");
});

fn roll(message: &String) -> String {
	//Stop if the message isn't an ASCII string
	//This is because I haven't found an easy way to slice a Unicode string
	if !message.is_ascii() {
		return String::from("Failed to roll: Message is not ASCII");
	}
	
	//This will be filled with three i32s. In order, these i32s are: the number of dice to roll, the number of faces, and the modifier
	let args: Vec<i32>;
	
	//Find the first space in the string. If there's not, that means the function was called with no arguments, meaning it should use the default roll of 1d20+0
	let first_space_location = message.find(' ').unwrap_or(0);
	
	if first_space_location == 0 {
		args = vec!(1, 20, 0);
	} else {
		//If it finds a space, convert to lowercase, strip all spaces, and go to the parse_args function to convert it into a Vec<i32>
		let unparsed_args = message.split_at(message.find(' ').unwrap_or(0)).1
			.to_ascii_lowercase()
			.replace(" ", "");
			
		args = parse_args(unparsed_args);
	}
	
	//If either the number of dice is negative, bail out early
	if args[0] < 1 {
		return String::from("Failed to roll: Number of dice is negative");
	}
	
	//Do the same with the number of faces
	if args[1] < 1 {
		return String::from("Failed to roll: Number of faces is negative");
	}

    //Fill the vector with dice rolls
    let mut roll_results = String::new();
	
	roll_results.push('`');
	
	//The final total of numbers that will be displayed at the end of the function
	let mut total: i32 = 0;

    for i in 0 .. args[0] {
		let generated_number = generate_number(args[1]);
		total += generated_number;
		
		let converted_number = generated_number.to_string();
        roll_results.push_str(&converted_number[..]);
		
		if i < args[0] - 1 {
			roll_results.push_str(" + ");
		}
    }

    if args[2] != 0 {
		roll_results.push(' ');
        roll_results.push('(');

        if args[2] > 0 {
			roll_results.push('+');
		}
			
		total += args[2];

		let converted_args = args[2].to_string();
        roll_results.push_str(&converted_args[..]);

        roll_results.push(')');
    }

    roll_results.push('`');

    //Combine roll_results and total together to form one message
    format!("{}. Total: {}", roll_results.as_str(), total)
}

fn parse_args(unparsed_args: String) -> Vec<i32> {
	let args: &str = &unparsed_args[..];
	
    let mut roll_data: Vec<&str> = args.split("d").collect();
	
	let mut parsed_args: Vec<i32> = Vec::new();

	let mut is_modifier_negative = false;

    //This is extremely messy, but it was the only way I could think to get it to work
    if roll_data.len() >= 2 {
		//Attempt to split on the +
        let mut modifier_data: Vec<&str> = roll_data[1].split("+").collect();
		
		//if that fails, split on the -
        if modifier_data.len() < 2 {
            modifier_data = roll_data[1].split("-").collect();
			//Mark it as negative as well
			is_modifier_negative = true;
        }
		
		//If both splits fail to find anything, set the modifier to 0. Otherwise, set it to whatever was after the + or -
        if modifier_data.len() < 2 {
            roll_data.push("0");
        } else {
            roll_data.push(modifier_data[1]);
            roll_data[1] = modifier_data[0];
        }
    }
	
	//Parse all the strings into their i32 counterparts
	//TODO: Replace all of this with unwrap_or, and find a way to make the bot say an error message if it's using a default variable
    match roll_data.len() {
        1 => {
			parsed_args.push(1);
			parsed_args.push(roll_data[0].parse::<i32>().unwrap());
			parsed_args.push(0);
        },

        2 => {
            parsed_args.push(roll_data[0].parse::<i32>().unwrap());
			parsed_args.push(roll_data[1].parse::<i32>().unwrap());
			parsed_args.push(0);
        }

        3 => {
            parsed_args.push(roll_data[0].parse::<i32>().unwrap());
			parsed_args.push(roll_data[1].parse::<i32>().unwrap());

			let modifier = roll_data[2].parse::<i32>().unwrap();

			//If the modifier is negative, convert the i32 to negative
			if is_modifier_negative {
				parsed_args.push(modifier - (2 * modifier));
			} else {
				parsed_args.push(modifier);
			}
        }

        _ => {
			//This shouldn't be possible, but just in case, panic
            panic!("roll_data.len() is outside of expected range");
        }
    }
	
	parsed_args
}

fn roll_fudge(message: &String) -> String {
	if !message.is_ascii() {
		return String::from("Failed to roll: Message is not ASCII");
	}

	let number_of_dice = match message.len() {
        //If the command is just "%r" with no arguments (Discord removes trailing spaces), roll 4 dice,
        //since that's the most common number of dice to roll in Fate.
        2 => 4,
        _ => {
            //Get the rest of the message, post-%r, as a slice.
            //If that slice is a valid u32, use that as the number of dice.
            //Else, use 4.
            message.as_str()[3..].parse::<i32>().unwrap_or(4)
        }
	};

	//Fill the vector with dice rolls.
    let mut roll_results = Vec::new();

    for _i in 0 .. number_of_dice {
		let mut rng = rand::thread_rng();

        roll_results.push(rng.gen_range::<i32>(1, 4) - 2);
	}

	//total is the number that will be printed, showing all the dice added together.
    let mut total: i32 = 0;

    //dice_rolls is the visual display, showing the results of each individual die.
    let mut dice_rolls = String::new();

	dice_rolls.push('`');

	for i in roll_results.iter() {
        match i {
            &-1 => {
                total -= 1;
                dice_rolls.push('-');
            },
            &0 => {
                dice_rolls.push(' ')
            },
            &1 => {
                total += 1;
                dice_rolls.push('+');
            },
            _ => {
				panic!("invalid roll result generated!");
			}
        }
	}

	dice_rolls.push('`');

	format!("{}. Total: {}", dice_rolls.as_str(), total)
}

fn generate_number(max: i32) -> i32 {
	//Get the thread's RNG
    let mut rng = rand::thread_rng();

    //max has to be increased by one, because rand never seems to generate the highest possible number given
    let result: i32 = rng.gen_range::<i32>(1, max + 1);

    //Panic if that happens though
    if result >= (max + 1) {
        panic!("Function returned a number too high!");
    }

    result
}
