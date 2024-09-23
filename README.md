# osui

`osui` is a Text User Interface (TUI) library written in Rust. 

# Please Read
OSUI is still in early development. Which means that the way you interact with OSUI will change.
Currently we are writing OSUI in Go and we will make a API that can be used by many popular languages like rust

Here is the OSUI Go version: https://github.com/orus-dev/osui

## Features

- **Cross-Platform Support**: Works on various operating systems.
- **Flexible Layout**: Easily create and manage complex UIs.
- **Efficient Performance**: Designed for high performance and low latency.

## Installation

To use `osui` in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
osui = "^0.0.2"
```

```rust
use osui::{self, render_frame, ui};

fn main() {
    let txt = ui::text("Hello World!");
    render_frame!(txt);
}
```

