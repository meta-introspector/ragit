use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::{Deserialize, Serialize};

mod browser;
pub use browser::BrowserExtractor;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! RAGIT is ready.", name));
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct WebChunk {
    pub content: String,
    pub title: String,
    pub url: String,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    pub chunks: Vec<WebChunk>,
    pub query: String,
}

#[wasm_bindgen]
pub struct RagitWeb {
    chunks: Vec<WebChunk>,
}

#[wasm_bindgen]
impl RagitWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RagitWeb {
        console::log_1(&"RAGIT WebAssembly initialized".into());
        RagitWeb {
            chunks: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_page(&mut self, content: &str, title: &str, url: &str) {
        let chunk = WebChunk {
            content: content.to_string(),
            title: title.to_string(),
            url: url.to_string(),
            summary: None,
        };
        self.chunks.push(chunk);
        console::log_1(&format!("Added page: {}", title).into());
    }

    #[wasm_bindgen]
    pub fn query(&self, query_text: &str) -> JsValue {
        // Simple keyword matching for now
        let matching_chunks: Vec<&WebChunk> = self.chunks
            .iter()
            .filter(|chunk| {
                chunk.content.to_lowercase().contains(&query_text.to_lowercase()) ||
                chunk.title.to_lowercase().contains(&query_text.to_lowercase())
            })
            .collect();

        let result = QueryResult {
            chunks: matching_chunks.into_iter().cloned().collect(),
            query: query_text.to_string(),
        };

        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"RAGIT WASM module loaded".into());
}