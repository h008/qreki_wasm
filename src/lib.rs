extern crate wasm_bindgen;
extern crate qreki;
use wasm_bindgen::prelude::*;
use qreki::Qreki;

#[wasm_bindgen]
pub fn from_ymd(y:i32,m:u32, d:u32) -> JsValue{
   let qreki =  Qreki::from_ymd(y,m,d);
   JsValue::from_serde(&qreki).unwrap()
}
