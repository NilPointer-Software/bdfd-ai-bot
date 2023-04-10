# BDFD AI Mod
![Screenshot of Discord chat with bot sending a support message](/media/bot_responding.png?raw=true)    

Our tiny Discord ticket support bot that utilizes the OpenAI GPT-3.5-turbo model.  
This project aims to help users by providing a very fast first response that will allow solving the most common issues.


## Running bot locally
Currently, the bot uses three environment variables:  
`OPEN_AI_TOKEN` - Token for OpenAI Api  
`DISCORD_BOT_TOKEN` - Discord bot token  
`CHANNEL_PARENT_ID` - ID of chat category that contains tickets  
  
We will probably switch to a more robust config solution in the future.

## Contributing
Please follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) for merge requests and commit 
names.    
If you want to contribute just to GPT prompts, you can find them in the `data/ai/prompts` project directory.



