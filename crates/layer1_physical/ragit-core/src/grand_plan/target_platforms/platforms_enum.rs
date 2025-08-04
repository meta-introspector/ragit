use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub enum TargetPlatform {
    Solana,
    WasmBrowser,
    WasmUserscript,
    WasmChromePlugin,
    DioxusWeb,
    EbpfKernel,
    Gpu,
}
