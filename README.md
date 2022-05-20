# dailykaenguru
Liefert den täglichen Känguru-Comic von Zeit Online auf Telegram.

This is a telegram bot to deliver the [Känguru-Comic](https://www.zeit.de/serie/die-kaenguru-comics) published every day by german newspaper [Zeit Online](https://www.zeit.de) directly in the messenger.

## Installation
Make sure you have a recent [rust toolchain](https://www.rust-lang.org/tools/install) installed.
Additionally, OpenSSL development libraries are required:

```sh
# Debian/Ubuntu based distros
apt install libssl-dev 

# RedHat/Fedora based distros
dnf install openssl-devel
```

You can then clone this repository and build/install a binary from source using cargo:

```sh
git clone https://github.com/jvytee/dailykaenguru.git
cargo install --path dailykaenguru
```

## Usage
Run `dailykaenguru` to start the telegram bot.
It can be easily configured via environment variables:

```sh
# Use /var/lib/dailykaenguru as cache directory
export DAILYKAENGURU_DATA=/var/lib/dailykaenguru

# Set secret telegram bot token
export DAILYKAENGURU_TOKEN=123:topsecret

dailykaenguru
```

To deliver the comic to all users that signed up, run `dailykaenguru --deliver`.
E.g. You could do so every morning at 09:00 using SystemD timers or any scheduling solution you like.

Running `dailykaenguru --download` will only acquire the latest comic from the internet.

You can furthermore set the `RUST_LOG` environment variable to [adjust log verbosity](https://docs.rs/env_logger).
