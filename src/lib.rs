use serde::{ Serialize, Deserialize };
use serde_wasm_bindgen::to_value;

pub mod levenshtein;
pub mod vptree;

mod utils;

use wasm_bindgen::prelude::*;
use crate::vptree::VPTree;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_error_panic_hook::set_once();
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub id: usize,
    pub connected_nodes: Vec<usize>,
}

#[wasm_bindgen]
pub fn search_matches(vec1: Vec<String>, vec2: Vec<String>, radius: usize) -> Result<JsValue, JsValue> {
    let mut matches = Vec::new();

    let tree = if vec1.len() > vec2.len() {
        VPTree::new(&vec1)
    } else {
        VPTree::new(&vec2)
    };

    if vec1.len() > vec2.len() {
        for (id, item) in vec2.iter().enumerate() {
            let connected_nodes = tree.search(item, radius);
            matches.push(Match { id, connected_nodes });
        }
    } else {
        for (id, item) in vec1.iter().enumerate() {
            let connected_nodes = tree.search(item, radius);
            matches.push(Match { id, connected_nodes });
        }
    };

    to_value(&matches).map_err(|e| JsValue::from_str(&e.to_string()))
}