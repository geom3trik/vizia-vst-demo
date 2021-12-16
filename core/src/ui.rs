use std::sync::Arc;

use vizia::*;

use crate::{GainEffectParameters};

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

#[derive(Lens, Clone)]
pub struct Params {
    gain: Arc<GainEffectParameters>,
    changed: bool,
}

impl Data for Params {
    fn same(&self, other: &Self) -> bool {
        // TODO - Find this
        false
    }
}

#[derive(Debug)]
pub enum ParamChangeEvent {
    SetGain(f32),
}

impl Model for Params {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                ParamChangeEvent::SetGain(new_gain) => {
                    self.gain.amplitude.set(*new_gain);
                    self.changed ^= true;
                }
            }
        }
    }
}

pub fn plugin_gui(cx: &mut Context, params: Arc<GainEffectParameters>) {
    cx.add_theme(STYLE);

    Params {
        gain: params.clone(),
        changed: false,
    }.build(cx);

    VStack::new(cx, |cx|{
        Label::new(cx, "GAIN");

        //let map = DecibelMap::new(-12.0, 12.0, ValueScaling::Linear, DisplayDecimals::One, true);
        let map = GenericMap::new(0.0, 1.0, ValueScaling::Linear, DisplayDecimals::Two, None);
        //let normalized_default = map.db_to_normalized(0.0);
        Knob::new(cx, map.clone(), 1.0).on_changing(cx, |knob, cx|{
            cx.emit(ParamChangeEvent::SetGain(knob.normalized_value));
        });
        Binding::new(cx, Params::root, move |cx, params|{
            let amplitude = params.get(cx).gain.amplitude.get();
            Label::new(cx, &map.normalized_to_display(amplitude));
        });

    }).background_color(Color::rgb(80, 80, 80)).child_space(Stretch(1.0)).row_between(Pixels(10.0));
}