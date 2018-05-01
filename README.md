# D20 Dice Roller
A sort of sequel to [my previous Fudge dice roller](https://github.com/devvoid/fudge_dice_roller), created for a friend's DND campaign.

## Installing
1. Create a bot user, and get its key. Look up a bot-making tutorial if you have no idea how to do this.
2. Clone/download this repository.
3. In the bot's root directory (the same place as Cargo.toml), create a file named "bot.token"
4. Put your bot token in bot.token.
5. Build and run like you would any other Rust program.

Requirements for Discord-RS can be found [here](https://github.com/SpaceManiac/discord-rs/).

The bot is known to work on Linux Mint 18.2 and Windows 10, but everything here should work fine on older Windows versions, Mac OS X, or any other Linux distro.

If you don't want to deal with setup, you can use the version of the bot I run on my servers. Add it using [this](https://discordapp.com/oauth2/authorize?&client_id=440710365072982026&scope=bot&permissions=0) link. I would only recommend doing this if you can't get the bot to run yourself; this bot only runs while my computer is on, and because of that, it has a lot of downtime.

If you need any help, or have any questions, message me on Discord at Void#3651.

## Use

There are three ways of using the bot:

`$r 20` - Rolls one 20-sided dice.

`$r 2d20` - Rolls two 20-sided dice.

`$r 2d20+5` - Rolls two 20-sided dice, with a modifier of 5.

Obviously, the numbers can be substituted for any numbers you need. Spaces are completely ignored; `2     d 23` is the exact same as `2d23`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
