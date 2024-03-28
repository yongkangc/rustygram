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
    <img src="https://img.shields.io/crates/v/rustygram.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API%20coverage-Up%20to%206.4%20(inclusively)-green.svg">
  </a>

⚡rustygram is a minimal and blazing fast telegram notification framework using [Rust](https://www.rust-lang.org/). Abstracts away the Telegram API complexity so your app doesn't have to worry about the underlying implementation.

</div>

## Highlights

- Easily integrate rustygram into your rust application to quickly send messages to Telegram bots, groups, and channels.

- Send asynchronous notifications in a reliable way.

## API overview

- `create_bot` - create a bot instance consistsing of Telegram's bot token, and target chat_id
- `send_message` - call Telegram bot's API sendMessage to send message asynchronously

## Examples

### Send in simple way

```rust
fn main() {
 let instance = rustygram::create_bot("123456:123456", "-1000000");
 if let Err(_) = rustygram::send_message(&instance, "Hello world", None) {
  // error handling here...
 }
}
```

### Send in `MarkdownV2` or `HTML` style message

Send message in `MarkdownV2`

```rust
use rustygram::types::{SendMessageOption, SendMessageParseMode};

fn main() {
 let instance = rustygram::create_bot("16", "-1s00");
 let option = SendMessageOption { parse_mode: Some(SendMessageParseMode::MarkdownV2) };

 // note on two spaces at the end of the line for a new line in markdown
 if let Err(_) = rustygram::send_message(&instance,
r#"__Hello world__
`Tap to copy this text`
Visit my [website](https://yong-kang.super.site/)"#, Some(option)) {
  // error handling here...
 }
}
```

Send messsage in `HTML`

```rust
use rustygram::types::{SendMessageOption, SendMessageParseMode};

fn main() {
 let instance = rustygram::create_instance("189:blablabla", "-10");
 let option = SendMessageOption { parse_mode: Some(SendMessageParseMode::HTML) };

 if let Err(_) = rustygram::send_message(&instance,
r#"<u>Hello world</u>
<code>Tap to copy this text</code>
Visit my <a href="https://yong-kang.super.site/">website</a>"#, Some(option)) {
  // error handling here...
 }
}
```

### Setting up and testing it as a class
- Check out [example.rs](https://github.com/yongkangc/rustygram/blob/main/example.rs) where there is a concrete example with tests

## Setting up your environment

1. [Download Rust](http://rustup.rs/).
2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `189:blablabla`.
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

You can test by define the following two environment variables

- `TEST_BOT_TOKEN` - telegram bot's token
- `TEST_CHAT_ID` - telegram bot's chat id

then execute

`cargo test`

some tests will send a single, or multiple messages to a specified chat id on behalf
of such telegram bot. Please take a look at `src/tests.rs`.

### Note

You can utilize this telegram bot `@username_to_id_bot` to get your
telegram channel's `chat_id`.

## Contributing

See [`CONRIBUTING.md`](CONTRIBUTING.md).

## Acknowledgements

This project is heavily inspired by [teloxide](https://github.com/ExtremelySunnyYk/rustygram). However it is not a fork of teloxide, but a complete rewrite in order to make it more minimal and faster for the use case of sending notifications on telegram.

The simplistic design is inspired by [rustelebot](https://github.com/haxpor/rustelebot) but with more modular and extensible design, also changing the underlying libraries used to be more modern.

## License

MIT, [Chia Yong Kang](https://www.linkedin.com/in/chiayong-eth/)
