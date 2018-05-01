extern crate discord;
extern crate rand;

use discord::{Discord, State};
use discord::model::{Event, Message};

use rand::Rng;

fn main() {
    let discord = Discord::from_bot_token(
        include_str!("../bot.token")
    ).expect("Failed to login with provided token.");

    let (mut connection, ready) = discord.connect().expect("Failed to connect.");

    let mut state = State::new(ready);

    println!("Connected.");

    loop {
        let event = match connection.recv_event() {
            Ok(event) => {
                event
            },

            //TODO: Some error handling here would be beneficial.
            _ => { continue }
        };

        state.update(&event);

        match event {
            Event::MessageCreate(message) => {
                //Skip if the bot sends the message.
                //Completely useless in this example, honestly.
                if message.author.id == state.user().id {
					continue
                }

				if &message.content[0 .. 2] == "%r" {
                    let dice_str: String;

                    if message.content.len() <= 2 {
                        dice_str = "1d20".to_string();
                    }
                    else {
                        dice_str = message.content[3..].replace(" ", "");
                    }

                    roll(&discord, &message, &dice_str.as_str());
                }
            }

            _ => {}
        }
    }
}

fn roll(client: &Discord, message: &Message, dice_str: &str) {
    //Get the thread's RNG.
    //You could probably move this to the generate_number function with no penalty,
    //but I'm not sure if repeatedly getting it from this function would incur overhead.
    let mut rng = rand::thread_rng();

    let mut roll_data: Vec<&str> = dice_str.split("d").collect();

    //This is extremely messy, but it was the only way I could think to get it to work.
    if roll_data.len() >= 2 {
        let mut modifier_data: Vec<&str> = roll_data[1].split("+").collect();
        if modifier_data.len() < 2 {
            modifier_data = roll_data[1].split("-").collect();
        }

        if modifier_data.len() < 2 {
            roll_data.push("0");
        } else {
            roll_data.push(modifier_data[1]);
            roll_data[1] = modifier_data[0];
        }
    }

    let number_of_dice: u32;
    let dice_maximum: u32;
    let modifier: i32;

    match roll_data.len() {
        1 => {
            number_of_dice = 1;
            dice_maximum = roll_data[0].parse::<u32>().unwrap();
            modifier = 0;
        },

        2 => {
            number_of_dice = roll_data[0].parse::<u32>().unwrap();
            dice_maximum = roll_data[1].parse::<u32>().unwrap();
            modifier = 0;
        }

        3 => {
            number_of_dice = roll_data[0].parse::<u32>().unwrap();
            dice_maximum = roll_data[1].parse::<u32>().unwrap();
            modifier = roll_data[2].parse::<i32>().unwrap();
        }

        _ => {
            panic!("roll_data.len() is outside of expected range");
        }
    }

    //Fill the vector with dice rolls.
    let mut roll_results = Vec::new();

    for _i in 0 .. number_of_dice {
        roll_results.push(generate_number(&mut rng, dice_maximum));
    }

    //total is the number that will be printed, showing all the dice added together.
    let mut total: i32 = 0;

    //dice_rolls is the visual display, showing the results of each individual die.
    let mut dice_rolls = String::new();

    dice_rolls.push('`');

    let mut numbers_added = 0;

    for i in roll_results.iter() {
        let temporaryvalue = i.to_string();

        dice_rolls.push_str(&temporaryvalue[..]);

        total += *i as i32;
        numbers_added += 1;

        if numbers_added < number_of_dice {
            dice_rolls.push_str(" + ");
        }
    }

    if modifier != 0 {
        dice_rolls.push('(');

        if modifier > 0 {
            dice_rolls.push('+');
        }

        dice_rolls.push_str(roll_data[2]);

        dice_rolls.push(')');

        total += modifier;
    }

    dice_rolls.push('`');

    //Combine dice_rolls and total together to form one message.
    let response = format!("{}: {}. Total: {}", message.author.mention(), dice_rolls.as_str(), total);

    //Send that message.
    client.send_message(message.channel_id, response.as_str(), "", false).expect("Failed to send message");
}

fn generate_number(rng: &mut rand::ThreadRng, max: u32) -> u32 {
    //Between 1 and 4 because for some reason, it seems like rand won't generate the highest number given.
    //So if it's set to 1 and 3, it's always a 1 or 2 it generates.
    let result: u32 = rng.gen_range::<u32>(1, max + 1);

    //For the reasons listed above, this should never happen, but if a future version of the rand crate changes the behavior,
    //this would let me know ahead of time.
    if result >= (max + 1) {
        panic!("Function returned a number too high!");
    }

    result
}
