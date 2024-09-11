# cli-clock
A terminal clock/stopwacth for the terminal, inspired by [tty-clock](https://github.com/xorg62/tty-clock).

## Installation
Building from source
```
git clone https://github.com/Archerymystery/cli-clock.git
cargo build --release 
./target/release/cli-clock 
```

## Usage
```
cli clock/stopwacth

Usage: cli-clock [OPTIONS]

Options:
  -c                     Center a clock
  -S                     Stopwatch mode
  -r                     Display in 12 hour clock format
  -s                     Display seconds
  -C, --char <CHAR>      Change char in the clock [default: █]
  -H, --hex <HEX>        Change clock color [default: #FFFFFF]
  -F, --format <FORMAT>  Date format
  -h, --help             Print help
  -V, --version          Print version
```
- `q` or `Q` to exit
