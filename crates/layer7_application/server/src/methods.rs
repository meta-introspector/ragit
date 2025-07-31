// use crate::error::Error;
// use crate::utils::trim_long_string;
// use ragit_fs::write_log;
// use sqlx::postgres::{PgPool, PgPoolOptions};
// use std::collections::HashMap;
// use std::str::FromStr;
// use warp::http::status::StatusCode;
// use warp::reply::{Reply, with_header, with_status};

// mod admin;
// mod ai_model;
// mod auth;
// mod chat;
// mod chunk;
// mod clone;
// mod health;
// mod image;
// mod push;

// // If an api has to read/write to disk storage (e.g. `index.json`), that's in `repo_fs` module.
// // If an api only reads/writes to DB, that's in `repo` module.
// mod repo;
// mod repo_fs;

// mod search;
// mod user;

// pub use admin::{drop_all, truncate_all};
// pub use ai_model::{get_ai_model_list, put_ai_model_list};
// pub use auth::{create_api_key, get_api_key_list};
// pub use chat::{create_chat, get_chat, get_chat_list, post_chat};
// pub use chunk::{get_chunk, get_chunk_count, get_chunk_list, get_chunk_list_all};
// pub use clone::{get_archive, get_archive_list};
// pub use health::get_health;
// pub use image::{get_image, get_image_desc, get_image_list};
// pub use push::{post_archive, post_begin_push, post_finalize_push};
// pub use repo::{get_repo, get_repo_list, get_traffic, put_repo};
// pub use repo_fs::{
//     create_repo, get_cat_file, get_config, get_content, get_file_content, get_index, get_meta,
//     get_meta_by_key, get_prompt, get_uid, get_version, post_build_search_index,
// };
// pub use search::search;
// pub use user::{
//     create_user, get_user, get_user_ai_model_list, get_user_list, put_user_ai_model_list,
// };

// static POOL: tokio::sync::OnceCell<PgPool> = tokio::sync::OnceCell::const_new();

pub async fn get_pool() -> &'static () {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub type RawResponse = Result<Box<dyn Reply>, (u16, String)>;

pub fn not_found() -> Box<dyn Reply> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub fn get_server_version() -> Box<dyn Reply> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub fn handler(_r: RawResponse) -> Box<dyn Reply> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub trait HandleError<T> {
    fn handle_error(self, code: u16) -> Result<T, (u16, String)>;
}

impl<T, E: std::fmt::Debug> HandleError<T> for Result<T, E> {
    fn handle_error(self, _code: u16) -> Result<T, (u16, String)> {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    }
}

impl<T> HandleError<T> for Option<T> {
    fn handle_error(self, _code: u16) -> Result<T, (u16, String)> {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    }
}

impl HandleError<()> for bool {
    fn handle_error(self, _code: u16) -> Result<(), (u16, String)> {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    }
}

pub fn check_secure_path(_path: &str) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

pub(crate) fn get_or<T: std::str::FromStr>(
    _query: &std::collections::HashMap<String, String>,
    _key: &str,
    _default_value: T,
) -> T {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}