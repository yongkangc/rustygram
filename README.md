<div align="center">
  <img src="./media/logo.png" width="250"/>
  <h1><code>⚡rustygram</code></h1>
  <a href="https://docs.rs/rustygram/">
    <img src="https://docs.rs/teloxide/badge.svg">
  </a>
  <a href="https://github.com/ExtremelySunnyYk/rustygram/actions">
    <img src="https://github.com/ExtremelySunnyYk/rustygram/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/rustygram">
    <img src="https://img.shields.io/crates/v/teloxide.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API%20coverage-Up%20to%206.4%20(inclusively)-green.svg">
  </a>

⚡rustygram is a minimal and blazing fast telegram notification framework using [Rust](https://www.rust-lang.org/). Abstracts away the Telegram API complexity so your app doesn't have to worry about the underlying implementation.

</div>

## Highlights

Easily integrate rustygram into your app to quickly send messages to Telegram bots, groups, and channels if you have the chat ID. Get up and running fast by focusing just on your app's specific messaging needs rather than Telegram API intricacies.

## API overview

- `create_instance` - create a bot instance consistsing of Telegram's bot token, and target chat_id
- `send_message` - call Telegram bot's API sendMessage to send message synchronously
- `send_message_async` - call Telegram bot's API sendMessage to send message asynchronously

## Examples

## Setting up your environment

1. [Download Rust](http://rustup.rs/).
2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
3. Initialise the `BOT_TOKEN` environmental variable to your token:

```bash
# Unix-like
$ export BOT_TOKEN=<Your token here>

# Windows command line
$ set BOT_TOKEN=<Your token here>

# Windows PowerShell
$ $env:BOT_TOKEN=<Your token here>
```

4. Make sure that your Rust compiler is up to date (`rustygram` currently requires rustc at least version 1.68):

```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

5. Run `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:

```toml
[dependencies]
rustygram = "0.1"
log = "0.4"
pretty_env_logger = "0.4"
tokio = { version =  "1.8", features = ["rt-multi-thread", "macros"] }
```

## Tests

## Contributing

See [`CONRIBUTING.md`](CONTRIBUTING.md).

## Acknowledgements

This project is heavily inspired by [teloxide](https://github.com/ExtremelySunnyYk/rustygram). However it is not a fork of teloxide, but a complete rewrite in order to make it more minimal and faster for the use case of sending notifications on telegram.

The functionalities are also inspired by [rustelebot](https://github.com/haxpor/rustelebot) which has been a joy to read.

## License

MIT, [Chia Yong Kang](https://www.linkedin.com/in/chiayong-eth/)
