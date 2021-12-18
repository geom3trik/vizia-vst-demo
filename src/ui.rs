use std::sync::Arc;

use vizia::*;

use crate::GainEffectParameters;

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

#[derive(Lens)]
pub struct Params {
    gain: Arc<GainEffectParameters>,
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
                }
            }
        }
    }
}

pub fn plugin_gui(cx: &mut Context, params: Arc<GainEffectParameters>) {
    cx.add_theme(STYLE);

    Params {
        gain: params.clone(),
    }.build(cx);

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
}