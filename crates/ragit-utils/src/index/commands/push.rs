use crate::error::Error;
use crate::index::commands::archive::erase_lines;
use crate::index::index_struct::Index;
use crate::index::commands::get_ragit_api_key::get_ragit_api_key;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PushResult {
    pub success: usize,
    pub errors: usize,
}

