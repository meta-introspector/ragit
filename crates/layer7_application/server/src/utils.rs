// use crate::CONFIG;
// use crate::error::Error;
// use bytes::BufMut;
// use futures_util::TryStreamExt;
// use ragit_fs::{FileError, join4};
// use std::collections::HashMap;
// use warp::filters::multipart::FormData;

pub async fn fetch_form_data(_form: ()) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

// ROOT/{user}/{repo}/.ragit
pub fn get_rag_path(_user: &str, _repo: &str) -> Result<String, anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub fn trim_long_string(_s: &str, _prefix_len: usize, _suffix_len: usize) -> String {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}