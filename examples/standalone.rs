
use std::sync::Arc;

use vizia::*;

use gain_vst_core::*;


fn main() {

    let params = Arc::new(GainEffectParameters::default());

    let window_description = WindowDescription::new().with_inner_size(300, 300);
    Application::new(window_description, move |cx|{

        plugin_gui(cx, params.clone());

    }).run();
}