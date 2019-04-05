use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! log {
    ( $( $arg:expr ),* ) => {
        crate::utils::log(&format!($($arg,)*));
    };
}
