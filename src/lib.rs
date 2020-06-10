
mod utils;

use wasm_bindgen::prelude::*;
extern crate js_sys;
use js_sys::Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
// x, y, h, m1x, m1y, m2x, m2y
		p1: Vec<i32>,
		p2: Vec<i32>,
}

#[wasm_bindgen]
impl Game {
		pub fn new() -> Game {
				let p1 = vec![50, 20, 100, -1, -1, -1, -1];
				let p2 = vec![50, 180, 100, -1, -1, -1, -1];

				Game {
						p1,
						p2,
				}
		}
		pub fn get_p1(&self) -> Array {
				self.p1.clone().into_iter().map(JsValue::from).collect()
		}

		pub fn get_p2(&self) -> Array {
				self.p2.clone().into_iter().map(JsValue::from).collect()
		}
}


