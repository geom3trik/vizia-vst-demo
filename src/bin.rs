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