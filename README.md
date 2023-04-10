# BDFD AI Mod

![Screenshot of Discord chat with bot sending a support message](/media/bot_responding.png?raw=true)

Our tiny Discord ticket support bot that utilizes the OpenAI GPT-3.5-turbo model. \
This project aims to help users by providing a very fast first response that will allow for solving the most common issues in an instant!

## Running bot locally

Currently, the bot uses three environment variables:
- `OPEN_AI_TOKEN` - Token for OpenAI API
- `DISCORD_BOT_TOKEN` - Discord bot token
- `CHANNEL_PARENT_ID` - Channel category ID containing tickets

In the future, we will switch to a more robust configuration solution.

## Contributing

If you want to contribute just GPT prompts, you can find them in the [`data/ai/prompts`](src/data/ai/prompts) directory.

Please follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) for merge requests and commit messages.
