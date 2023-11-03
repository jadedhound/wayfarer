use js_sys::Uint8Array;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use super::DBError;

pub fn rust_to_js<V>(value: V) -> Result<JsValue, DBError>
where
    V: Serialize,
{
    let code = bincode::serialize(&value)?;
    let js_value = JsValue::from(js_sys::Uint8Array::from(&code[..]));
    Ok(js_value)
}

pub fn rust_from_js<V>(value: JsValue) -> Result<V, DBError>
where
    V: DeserializeOwned,
{
    let code = Uint8Array::from(value).to_vec();
    Ok(bincode::deserialize::<V>(&code)?)
}
