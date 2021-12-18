# Part 2 - Defining the Plugin Parameters

In this part we will create some of the shared types which will be used by both the UI code and the plugin.

Add a `dsp.rs` file to the `src` directory. Then, add the following code:

```rust
use vst::{util::AtomicFloat, plugin::PluginParameters};

// The parameters of the plugin
pub struct GainEffectParameters {
    pub amplitude: AtomicFloat,
}

impl Default for GainEffectParameters {
    fn default() -> GainEffectParameters {
        GainEffectParameters {
            amplitude: AtomicFloat::new(1.0),
        }
    }
}

impl PluginParameters for GainEffectParameters {
    // the `get_parameter` function reads the value of a parameter.
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.amplitude.get(),
            _ => 0.0,
        }
    }

    // the `set_parameter` function sets the value of a parameter.
    fn set_parameter(&self, index: i32, val: f32) {
        match index {
            0 => self.amplitude.set(val),
            _ => (),
        }
    }

    // This is what will display underneath the control produced by the host
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", (self.amplitude.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    // This shows the control's name
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Amplitude",
            _ => "",
        }
        .to_string()
    }
}
```

Now to explain the code. The fist part is a struct definition which holds the plugin parameters. For this plugin there is a single parameter, `amplitude`, and we use an `AtomicFloat` type (provided by the `vst` crate). Using atomics allows us to easily share parameters between the UI thread and the audio thread where the audio processing will take place.

The next part implements `Default` for our parameter struct, giving the `amplitude` parameter an default value of `1.0`.

Lastly we implement the `PluginParameters` trait on our parameters struct. This contains methods which will be called by the host to manipulate and display the plugins parameters.

In the next part we will create the plugin and write the processing code without the GUI elements to begin with.

