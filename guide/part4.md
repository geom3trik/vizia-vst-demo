# Part 4 - Adding the VIZIA GUI

We're finally ready to write some VIZIA code to create a GUI for our simple plugin.

Add a new file called `ui.rs` in the `src` directory.

VIZIA is a 'reactive' GUI lib, which means that the UI elements change in response to changes in some data. Unfortunately, we can't react directly to the `GainEffectParameters`, because this is owned by the plugin. So first we must create a wrapper type which can be built into the VIZIA GUI:

```rust
#[derive(Lens)]
pub struct Params {
    gain: Arc<GainEffectsParameters>
}
```

This struct also contains a pointer to the plugin parameters. Also note the `Lens` derive which we'll come back to later. For mutating data from the UI, VIZIA uses an event system. Events are propagated up the visual tree of widgets to the data and changes in data are propagated down to widgets which our bound to that data. We'll come back to the binding part in a moment, but for now we need to define a type to describe the events:

```rust
#[derive(Debug)]
pub enum ParamChangeEvent {
    SetGain(f32),
}
```

With our data and events we can now implement the `Model` trait on the data type which has an `event` method which is used to respond to our custom event and update the data. In this case to update the plugin parameters:

```rust
impl Model for Params {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                ParamChangeEvent::SetGain(new_gain) => {
                    self.gain.amplitude.set(*new_gain);
                }
            }
        }
    }
}
```

This might seem quite verbose up to this point but using events to mutate data was a purposeful choice to force a separation between data and UI.

Finally, we can build the UI elements/widgets using the declarative API which VIZIA provides. Start by creating a function which takes a mutable reference to the vizia `Context`, and the plugin parameters:

```rust
pub fn plugin_gui(cx: &mut Context, params: Arc<GainEffectParameters>) {
    
} 
```

Inside this function we'll build an instance of the `Params` data, using a copy of the plugin parameters:

```rust
Params {
    gain: params.clone(),
}.build(cx);
```

Next, we'll delcare a `VStack` widget, which arranges its contents into a vertical stack:

```rust
VStack::new(cx, |cx|{

    ...

})
```

Inside the build closure of the `VStack` we'll first declare a `Label` with the text `'GAIN'`.

Then we'll create a `GenericMap` which is passed to a `Knob` widget. To mutate the data with the knob we need to emit our custom event when the knob is manipulated. For this we'll use the `on_changing` callback on the `Knob` which allows us to emit our event with the current value of the knob:

```rust
let map = GenericMap::new(0.0, 1.0, ValueScaling::Linear, DisplayDecimals::Two, None);

Knob::new(cx, map.clone(), 1.0).on_changing(cx, |knob, cx|{
    cx.emit(ParamChangeEvent::SetGain(knob.normalized_value));
});
```

Lastly we want another label to show the current value. To do this we need a `Binding` to the data. A binding lets us specify a piece of data using a lens and then allows us to use that data with widgets declared within the body of the binding. For this guide I'm not going to go too deeply into lenses, but in short they allow us to specify some field of a larger structure of data, in this case the `gain` field of the `Params` struct:

```rust
Binding::new(cx, Params::gain, move |cx, gain|{
    let amplitude = gain.get(cx).amplitude.get();
    Label::new(cx, &map.normalized_to_display(amplitude));
});
```

We're almost done. The very last thing we need to do is apply some styling to our widgets. We can do this in two ways. First, we'll add some style properties directly to the `VStack` widget to set its background color, position its children in the ceneter, and add some space between its children:

```rust
VStack::new(cx, |cx|{
    Label::new(cx, "GAIN");

    let map = GenericMap::new(0.0, 1.0, ValueScaling::Linear, DisplayDecimals::Two, None);

    Knob::new(cx, map.clone(), 1.0).on_changing(cx, |knob, cx|{
        cx.emit(ParamChangeEvent::SetGain(knob.normalized_value));
    });

    Binding::new(cx, Params::gain, move |cx, gain|{
        let amplitude = gain.get(cx).amplitude.get();
        Label::new(cx, &map.normalized_to_display(amplitude));
    });

}).background_color(Color::rgb(80, 80, 80)).child_space(Stretch(1.0)).row_between(Pixels(10.0));
```

And secondly we'll add some `css` to the top of the file:

```rust
const STYLE: &str = r#"

    label {
        font-size: 20;
        color: #C2C2C2;
    }

    knob {
        width: 70px;
        height: 70px;
    }
    
    knob .track {
        background-color: #ffb74d;
    }

"#;
```

And include it into the UI by adding `cx.add_theme(STYLE)` at the top of the function. This stylesheet will apply some basic styling to our labels and knob widets.

To test this code without having to build a plugin and open a host, delete the contents of the `bin.rs` file and add the following:

```rust
use std::sync::Arc;

use vizia::{WindowDescription, Application};

mod dsp;
use dsp::*;

mod ui;
use ui::*;

fn main() {
    let params = Arc::new(GainEffectParameters::default());

    let window_description = WindowDescription::new()
        .with_inner_size(300, 300)
        .with_title("Hello Plugin");

    Application::new(window_description, move |cx|{

        plugin_gui(cx, Arc::clone(&params));

    }).run();
}
```

Then run the application with:
```
cargo run --bin vizia_vst_demo
```

To add the UI code to the plugin, add the `ui` module to the `lib.rs` file:

```rust
mod ui;
use ui::*;
```

And then add the following to the `open` method of the `Editor` trait implementation in `lib.rs`:

```rust
let params = self.params.clone();

let window_description = WindowDescription::new()
    .with_inner_size(300, 300)
    .with_title("Hello Plugin");

Application::new(window_description, move |cx| {

    plugin_gui(cx, params.clone());

}).open_parented(&ParentWindow(parent));
```

Build the plugin with:
```
cargo build --lib
```

This concludes the guide for the basic gain plugin example with a VIZIA GUI.






