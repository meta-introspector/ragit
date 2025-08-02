# RAGIT WebAssembly Browser Extension

This crate compiles RAGIT's core functionality to WebAssembly for use in browser extensions and userscripts.

## Features

- **Universal Knowledge Base**: Turn any website into part of your searchable knowledge base
- **Instant Search**: Query your browsing history with RAGIT's TF-IDF powered search
- **Keyboard Shortcuts**: Quick access with Ctrl+Shift+A (add page) and Ctrl+Shift+R (search)
- **Local Storage**: All data stays in your browser's local storage

## Building

### Prerequisites
- Rust with `wasm32-unknown-unknown` target
- `wasm-pack` (will be installed automatically)

### Build Commands

**Windows:**
```cmd
build.bat
```

**Linux/Mac:**
```bash
chmod +x build.sh
./build.sh
```

## Installation

1. Build the extension using the commands above
2. Open Chrome and navigate to `chrome://extensions/`
3. Enable "Developer mode" in the top right
4. Click "Load unpacked" and select the `js/pkg/` directory

## Usage

### Adding Pages
- **Automatic**: The extension runs on all pages
- **Manual**: Click the extension icon and select "Add Current Page"
- **Keyboard**: Press `Ctrl+Shift+A`

### Searching
- **Extension Popup**: Click the extension icon and select "Search Knowledge Base"
- **Keyboard**: Press `Ctrl+Shift+R` on any page

### Query Interface
The search interface appears as an overlay on the current page, showing:
- Matching page titles (clickable links)
- Content previews
- Relevance-based ranking

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Web Pages     │───▶│  RAGIT WASM      │───▶│  Browser        │
│                 │    │  - Content       │    │  Storage        │
│  - Extract text │    │    extraction    │    │  - IndexedDB    │
│  - Metadata     │    │  - TF-IDF search │    │  - LocalStorage │
│  - URLs         │    │  - Chunking      │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## API

### RagitWeb
```rust
impl RagitWeb {
    pub fn new() -> RagitWeb;
    pub fn add_page(&mut self, content: &str, title: &str, url: &str);
    pub fn query(&self, query_text: &str) -> JsValue;
    pub fn get_chunk_count(&self) -> usize;
}
```

### BrowserExtractor
```rust
impl BrowserExtractor {
    pub fn new() -> Result<BrowserExtractor, JsValue>;
    pub fn extract_page_content(&self) -> Result<String, JsValue>;
    pub fn get_page_title(&self) -> String;
    pub fn get_page_url(&self) -> Result<String, JsValue>;
}
```

## Future Enhancements

- [ ] AI-powered summarization of web pages
- [ ] Cross-device synchronization
- [ ] Advanced filtering and tagging
- [ ] Integration with RAGIT desktop version
- [ ] Support for PDF and image content
- [ ] Collaborative knowledge sharing

## Userscript Version

For Tampermonkey/Greasemonkey, a userscript version can be generated that includes the WASM module inline.