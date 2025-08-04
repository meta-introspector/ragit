use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, Storage};

#[wasm_bindgen]
pub struct BrowserExtractor {
    document: Document,
    storage: Storage,
}

#[wasm_bindgen]
impl BrowserExtractor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<BrowserExtractor, JsValue> {
        let window = window().ok_or("No window object")?;
        let document = window.document().ok_or("No document object")?;
        let storage = window.local_storage()?.ok_or("No localStorage")?;
        
        Ok(BrowserExtractor { document, storage })
    }

    #[wasm_bindgen]
    pub fn extract_page_content(&self) -> Result<String, JsValue> {
        let body = self.document.body().ok_or("No body element")?;
        Ok(body.text_content().unwrap_or_default())
    }

    #[wasm_bindgen]
    pub fn get_page_title(&self) -> String {
        self.document.title()
    }

    #[wasm_bindgen]
    pub fn get_page_url(&self) -> Result<String, JsValue> {
        let window = window().ok_or("No window object")?;
        let location = window.location();
        Ok(location.href()?)
    }

    #[wasm_bindgen]
    pub fn save_to_storage(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.storage.set_item(key, value)
    }

    #[wasm_bindgen]
    pub fn load_from_storage(&self, key: &str) -> Result<Option<String>, JsValue> {
        self.storage.get_item(key)
    }
}