# Alfred

A terminal user interface (TUI) application built with [Ratatui](https://ratatui.rs).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.86 or newer (stable toolchain)

## Getting started

```bash
cargo run
```

### Key bindings

| Key                    | Action    |
| ---------------------- | --------- |
| `→` / `l` / `+`        | Increment |
| `←` / `h` / `-`        | Decrement |
| `r`                    | Reset     |
| `q` / `Esc` / `Ctrl+C` | Quit      |

## Development

```bash
cargo fmt --all          # format
cargo clippy --all-targets -- -D warnings   # lint
cargo test               # run tests
```

CI runs the same three checks on every push to `main` and on pull requests.

## Project layout

```
src/
├── main.rs   # Entry point: sets up error reporting and the terminal
├── app.rs    # Application state, main loop and event handling
└── ui.rs     # Rendering (pure: reads state, draws widgets)
```

The architecture follows a simple split between **state** (`App`), **events**
(`App::handle_events` / `App::on_key`) and **view** (`ui::render`). New screens
and widgets should keep rendering free of state mutation so they remain easy to
test with Ratatui's `TestBackend`.

## License

MIT — see [LICENSE](LICENSE).
