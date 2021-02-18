# ansi4tui

TUI doesn't natively support converting ANSI terminal codes to its own `Style` abstraction, so this crate provides a simple (and probably naive) conversion method. It has not been rigorously tested.


### Example

```rust
use std::process::Command;

let c = Command::new("ls")
    .args(&["--color=always", "~"])
    .output()
    .unwrap();

let text = ansi4tui::bytes_to_text(c.stdout);
```

For a simple usage example, see [the example file](./examples/simple.rs). You can run the example with `cargo run --example simple`, and close the display by pressing `q`. It assumes you're on a system with `/etc/hosts`, so change the file if you want to run the example and that file isn't available.
