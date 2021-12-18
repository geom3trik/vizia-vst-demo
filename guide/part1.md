

# Part 1 - Project Setup

To make development easier, we will structure our project to be able to build both a VST dynamic library and a standalone application using the same GUI code.

Start by creating a new Rst library project:

```
cargo new --lib vizia_vst_demo
```

Then, add a `bin.rs` file to the `src` directory and insert the following code into the file:

```rust
fn main() {
    println!("Hello World");
}
```

In the `cargo.toml` file add the following:

```
[lib]
name = "vizia_vst_demo"
path = "src/lib.rs"
crate-type = ["cdylib"]

[[bin]]
name = "vizia_vst_demo"
path = "src/bin.rs"
```

Now we can build the library with the following:
```
cargo build --lib
```

Or, build and run the binary file with:
```
cargo run --bin vizia_vst_demo
```

which should output 'Hello World' to the terminal.

Lastly we need to add the dependencies we'll need to build the VST. Add the following under `[dependencies]` in the `cargo.toml` file:

```
raw-window-handle = "0.3"
vst = "0.2.1"
vizia = {git = "https://github.com/geom3trik/VIZIA", branch = "main", features = ["baseview"], default-features = false}
dirs = "3"
log = "0.4"
log-panics = "2"
simplelog = "0.8"
```

We are now ready to start coding our VST plugin. Proceed to part 2 of this guide.
