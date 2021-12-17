# vizia_vst_demo
A tutorial (coming soon) and demo project for creating a simple VST plugin with a VIZIA GUI.

## Building the Plugin
To build the plugin library run:
```
cargo build --lib
```

## Building the Standalone Application
The demo can also run as a standalone application:
```
cargo run --bin vizia_vst_demo
```

The plugin and standalone versions share the same UI code which can e found in `ui.rs`.