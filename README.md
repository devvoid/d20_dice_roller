# D20 Dice Roller

**As of 2019, this repo is abandoned; use [the updated version on Gitlab](https://gitlab.com/devvoid/dicemaster) instead.**

A sort of sequel to my previous Fudge dice roller, created for a friend's DND campaign.

## Installing
You can download a premade build of the bot from the [Releases page](https://github.com/devvoid/d20_dice_roller/releases). Just put your bot's token in bot.token and you should be good to go.

If you need to compile it for yourself, you can do it like so:

1. Install OpenSSL; Serenity requires it.
2. Create a bot user, and get its key. Look up a bot-making tutorial if you have no idea how to do this.
3. Clone/download this repository.
4. In the bot's root directory (the same place as Cargo.toml), create a file named "bot.token"
5. Put your bot token in bot.token.
6. Build and run like you would any other Rust program.

If you would prefer to not have your bot's token compiled into the executable, compile with the flag `--features external_token`. This will instead make the bot load the token from a `bot.token` file in the same folder as itself.

If you don't want to deal with setup, you can use the version of the bot I run on my servers. Add it using [this](https://discordapp.com/oauth2/authorize?&client_id=440710365072982026&scope=bot&permissions=0) link. I would only recommend doing this if you can't get the bot to run yourself; this bot only runs while my computer is on, and because of that, it has a lot of downtime.

If you need any help, or have any questions, message me on Discord at Void#3651.

## Use

There are four ways of using the bot:

`$r` - Rolls one twenty-sided die.

`$r <number of faces>` - Rolls one die.

`$r <number of dice>d<number of faces>` - Rolls multiple dice.

`$r <number of dice>d<number of faces>+<modifier>` - Rolls multiple dice, with a modifier. The modifier can be positive or negative.

You can use either `$r` or `$roll`. The bot ignores spaces in the arguments.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
