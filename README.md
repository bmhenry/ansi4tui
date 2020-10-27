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
