# 🌃 Rua
A simplified Rust-implementation of D3vd/Meme_API. You can self-host this with ease through Docker, but you can also
opt to use our free service.

## 🌟 Endpoints
You can request for a single post using the format `/get/{subreddit}` which will respond
with a data structure such as this:
```json
{
  "data": {
    "id":"rfm7qv",
    "title":"Good luck EI [Genshin Impact]",
    "image":"https://i.redd.it/amkv5hicqc581.jpg",
    "ups":4054,
    "downs":0,
    "nsfw":false,
    "author":"raveenbikha"
  }
}
```

## 🧱 Self-hosting
You can self-host this by building the Docker image yourself through the
steps below:
1. Clone this repository through `git clone https://github.com/ShindouMihou/Rua`
2. Configure the `.env.example` file by renaming the `.env.example` to `.env` and setting the values inside.
3. Run the following command: `docker run -d -i -t --env-file .env -p 3921:3921 --name rua --restart unless-stopped rua`.
4. You can now use your self-hosted instance of Rua.
