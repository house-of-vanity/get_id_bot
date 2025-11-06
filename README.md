# Get ID Bot

Simple Telegram bot that replies with user ID.

## Usage

Set token and run:
```bash
export TELOXIDE_TOKEN=your_token
cargo run
```

Docker:
```bash
docker build -t get_id_bot .
docker run -e TELOXIDE_TOKEN=your_token get_id_bot
```
