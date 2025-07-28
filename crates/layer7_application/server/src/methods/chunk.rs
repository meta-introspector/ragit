use super::{HandleError, RawResponse, get_pool, handler};
use crate::models::repo::RepoOperation;
use crate::models::{chunk, repo};
use crate::utils::get_rag_path;
use ragit::chunk as ragit_chunk;
use ragit_fs::{
    exists, extension, file_name, join, join3, join4, read_dir, read_string, set_extension,
};
use serde_json::Value;
use warp::reply::{Reply, json};

pub async fn get_chunk_count(
    user: String,
    repo: String,
    api_key: Option<String>,
) -> Box<dyn Reply> {
    handler(get_chunk_count_(user, repo, api_key).await)
}

async fn get_chunk_count_(user: String, repo: String, api_key: Option<String>) -> RawResponse {
    let pool = get_pool().await;
    let repo_id = repo::get_id(&user, &repo, pool).await.handle_error(404)?;
    repo::check_auth(repo_id, RepoOperation::Read, api_key, pool)
        .await
        .handle_error(500)?
        .handle_error(404)?;

    let rag_path = get_rag_path(&user, &repo).handle_error(400)?;
    let index_path = join(&rag_path, "index.json").handle_error(400)?;
    let index_json = read_string(&index_path).handle_error(404)?;
    let index = serde_json::from_str::<Value>(&index_json).handle_error(500)?;

    let (code, error) = match index {
        Value::Object(obj) => match obj.get("chunk_count") {
            Some(Value::Number(n)) => match n.as_u64() {
                Some(n) => {
                    return Ok(Box::new(json(&n)));
                }
                _ => (500, format!("`{n:?}` is not a valid chunk_count")),
            },
            Some(x) => (500, format!("`{x:?}` is not a valid integer")),
            None => (500, format!("`{index_path}` has no field `chunk_count`")),
        },
        index => (500, format!("`{index:?}` is not a valid index")),
    };

    Err((code, error))
}

pub async fn get_chunk_list(
    user: String,
    repo: String,
    prefix: String,
    api_key: Option<String>,
) -> Box<dyn Reply> {
    handler(get_chunk_list_(user, repo, prefix, api_key).await)
}

async fn get_chunk_list_(
    user: String,
    repo: String,
    prefix: String,
    api_key: Option<String>,
) -> RawResponse {
    let pool = get_pool().await;
    let repo_id = repo::get_id(&user, &repo, pool).await.handle_error(404)?;
    repo::check_auth(repo_id, RepoOperation::Read, api_key, pool)
        .await
        .handle_error(500)?
        .handle_error(404)?;
    let rag_path = get_rag_path(&user, &repo).handle_error(400)?;

    if !exists(&rag_path) {
        return Err((404, format!("`{user}/{repo}` does not exist")));
    }

    let chunk_path = join3(&rag_path, "chunks", &prefix).handle_error(400)?;
    let chunks = read_dir(&chunk_path, false).unwrap_or(vec![]);
    Ok(Box::new(json(
        &chunks
            .iter()
            .filter_map(|chunk| match extension(chunk) {
                Ok(Some(e)) if e == "chunk" => file_name(chunk)
                    .ok()
                    .map(|suffix| format!("{prefix}{suffix}")),
                _ => None,
            })
            .collect::<Vec<String>>(),
    )))
}

pub async fn get_chunk_list_all(
    user: String,
    repo: String,
    api_key: Option<String>,
) -> Box<dyn Reply> {
    handler(get_chunk_list_all_(user, repo, api_key).await)
}

async fn get_chunk_list_all_(user: String, repo: String, api_key: Option<String>) -> RawResponse {
    let pool = get_pool().await;
    let repo_id = repo::get_id(&user, &repo, pool).await.handle_error(404)?;
    repo::check_auth(repo_id, RepoOperation::Read, api_key, pool)
        .await
        .handle_error(500)?
        .handle_error(404)?;
    let rag_path = get_rag_path(&user, &repo).handle_error(400)?;
    let chunk_parents = join(&rag_path, "chunks").handle_error(400)?;
    let mut result = vec![];

    for prefix in 0..256 {
        let prefix = format!("{prefix:02x}");
        let chunks_at = join(&chunk_parents, &prefix).handle_error(400)?;

        if exists(&chunks_at) {
            for chunk in read_dir(&chunks_at, false).unwrap_or(vec![]) {
                if extension(&chunk).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                    result.push(format!("{prefix}{}", file_name(&chunk).handle_error(500)?));
                }
            }
        }
    }

    Ok(Box::new(json(&result)))
}

pub async fn get_chunk(
    user: String,
    repo: String,
    uid: String,
    api_key: Option<String>,
) -> Box<dyn Reply> {
    handler(get_chunk_(user, repo, uid, api_key).await)
}

async fn get_chunk_(
    user: String,
    repo: String,
    uid: String,
    api_key: Option<String>,
) -> RawResponse {
    let pool = get_pool().await;
    let repo_id = repo::get_id(&user, &repo, pool).await.handle_error(404)?;
    repo::check_auth(repo_id, RepoOperation::Read, api_key, pool)
        .await
        .handle_error(500)?
        .handle_error(404)?;
    let rag_path = get_rag_path(&user, &repo).handle_error(404)?;
    let prefix = uid
        .get(0..2)
        .ok_or_else(|| format!("invalid uid: {uid}"))
        .handle_error(400)?
        .to_string();
    let suffix = uid
        .get(2..)
        .ok_or_else(|| format!("invalid uid: {uid}"))
        .handle_error(400)?
        .to_string();
    let chunk_path = join4(
        &rag_path,
        "chunks",
        &prefix,
        &set_extension(&suffix, "chunk").handle_error(400)?,
    )
    .handle_error(400)?;
    let chunk = ragit_chunk::load_from_file(&chunk_path).handle_error(404)?;
    let chunk = chunk::ChunkDetail::from(chunk);

    Ok(Box::new(json(&chunk)))
}
