# Part 3 - Constructing the Plugin

Using the `vst` crate, a plugin is made up of two user defined types:
 - A type which implements `Plugin` and is responsible for the audio processing aspect of the plugin
 - A type which implements `Editor` and is responsible for providing the graphical user interface of the plugin

In this part we will create both but leave the UI part empty until the next part of the guide. 

Start by including the necessary types from the `vst` crate at the top of the `lib.rs` file in the `src` directory:

```rust
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;
```

We'll also need to include `Arc` and our `dsp` module:

```rust
use std::sync::Arc;

mod dsp;
use dsp::*;
```

Then, define a struct with a name of your choice:
```rust
struct GainPluginEditor { 
    params: Arc<GainEffectsParameters>,
    is_open: bool,
}
```
This struct contains a reference counted pointer to an instance of the parameters, as well as a bool used to determine if the plugin editor window is open.

And implement the `Editor` trait on this type with the following methods:

```rust
struct GainPluginEditor {
    params: Arc<GainEffectParameters>,
    is_open: bool,
}

impl Editor for GainPluginEditor {

    // Determines the initial position of the plugin editor window
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    // Determines the initial size of the plugin editor window
    fn size(&self) -> (i32, i32) {
        (300, 300)
    }

    // Called by the host to open the plugin editor window
    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        if self.is_open {
            return false;
        }

        self.is_open = true;

        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    // Called by the host to close the plugin editor window
    fn close(&mut self) {
        self.is_open = false;
    }
}
```

For now we have left the `open` method mostly empty, but this is where our UI code will go later.

Next, create another type for the plugin:

```rust
struct GainPlugin {
    params: Arc<GainEffectParameters>,
    editor: Option<GainPluginEditor>,
}
```

This struct also contains a reference counted pointer to the parameters, as well as an instance of the editor struct. Next, we'll implement the `Default` trait for the `GainPlugin` type shown above:

```rust
impl Default for GainPlugin {
    fn default() -> Self {
        let params = Arc::new(GainEffectParameters::default());
        Self {
            params: params.clone(),
            editor: Some(GainPluginEditor {
                params: params.clone(),
                is_open: false,
            }),
        }
    }
}
```

As well as the `Plugin` trait, which is where the audio processing code will go:

```rust
impl Plugin for GainPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Vizia Gain Effect in Rust".to_string(),
            vendor: "Geom3trik".to_string(),
            unique_id: 243213073,
            version: 1,
            inputs: 2,
            outputs: 2,
            // This `parameters` bit is important; without it, none of our
            // parameters will be shown!
            parameters: 1,
            category: Category::Effect,
            ..Default::default()
        }
    }

    // This is called once when the plugin instance is created by the host
    // and is used here to setup some logging
    fn init(&mut self) {
        let log_folder = ::dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

        let log_file = ::std::fs::File::create(log_folder.join("vizia_vst_demo.log")).unwrap();

        let log_config = ::simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

        let _ = ::simplelog::WriteLogger::init(simplelog::LevelFilter::Info, log_config, log_file);

        ::log_panics::init();

        ::log::info!("init");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }

    // This is where the audio processing code will go
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Read the amplitude from the parameter object
        let amplitude = self.params.amplitude.get();
        // First, we destructure our audio buffer into an arbitrary number of
        // input and output buffers.  Usually, we'll be dealing with stereo (2 of each)
        // but that might change.
        for (input_buffer, output_buffer) in buffer.zip() {
            // Loop through each sample and apply the amplitude value to it
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = *input_sample * amplitude;
            }
        }
    }

    // Return the parameter object
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}
```

Finally, add the following to the end of the file:

```rust
plugin_main!(GainPlugin);
```

We can now build the VST plugin with the following command:

```
cargo build --lib
```

Copy the resulting VST file into the appropriate folder for your preferred host (DAW) and try adding it. The editor window which appears contains no UI but the host should generate somewhere a control for the amplitude. Adjusting this control should affect the volume of the audio output.

In the next part we'll add a VIZIA GUI to the plugin editor window.
