# zlog

A plain logger which writes to the console on a separate thread.

## Usage

```rust
extern crate zlog;
extern crate log;
use zlog::ConsoleLogger;

pub fn main() {
    console_logger::init(env!("LOG_LEVEL"));

}
```
