use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no graphics adapter found that suffices all the requirements")]
    NoAdapter,

    #[cfg(target_arch = "wasm32")]
    #[error("javascript error: {value:?}")]
    JsValue { value: wasm_bindgen::JsValue },

    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("json error")]
    Json(#[from] serde_json::Error),
}

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::JsValue { value }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<Error> for wasm_bindgen::JsValue {
    fn from(e: Error) -> Self {
        let s = e.to_string();
        log::error!("{}", s);
        s.into()
    }
}
