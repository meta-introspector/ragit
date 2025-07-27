pub mod chunk_creation_utils;

pub async fn create_chunk_from(
    tokens: &[AtomicToken],
    config: &BuildConfig,
    file: String,
    file_index: usize,
    api_config: &ApiConfig,
    pdl: &str,
    build_info: ragit_types::chunk::chunk_struct::ChunkBuildInfo,
) -> Result<Chunk, Error> {
    let mut dummy_context = tera::Context::new();
    dummy_context.insert("chunk", "placeholder");

    let Pdl { messages: mut prompt, .. } = ragit_pdl::parse_pdl(
        pdl,
        &dummy_context,
        &file, // Assuming 'file' can be used as the current directory for media files
        true, // strict_mode
    ).map_err(WrappedPdlError)?;

    if let Some(message) = prompt.last_mut() {
        debug_assert_eq!(message.role, Role::User);
        let content = tokens.iter().map(
            |content| MessageContent::from(content.clone())
        ).collect::<Vec<MessageContent>>();

        message.content = content;
    }

    else {
        unreachable!()
    }

    let messages = prompt;
    let model = Model { name: api_config.model.clone(), ..Default::default() };
    let temperature = None;
    let frequency_penalty = None;
    let max_tokens = None;
    let timeout = api_config.timeout;
    let max_retry = api_config.max_retry;
    let sleep_between_retries = api_config.sleep_between_retries;
    let dump_api_usage_at = api_config.dump_api_usage_at(&PathBuf::from(&file), "create_chunk_from");
    let dump_pdl_at = api_config.create_pdl_path(&PathBuf::from(&file), "create_chunk_from").map(|path| path.to_string_lossy().into_owned());
    let dump_json_at = None;
    let schema = None;
    let schema_max_try = 0;

    let started_at = std::time::Instant::now();

    let (response_text, title, summary) = chunk_creation_utils::send_and_validate_chunk_response(
        messages,
        model,
        temperature,
        frequency_penalty,
        max_tokens,
        timeout,
        max_retry,
        sleep_between_retries,
        dump_api_usage_at,
        dump_pdl_at,
        dump_json_at,
        schema,
        schema_max_try,
        started_at,
        config.min_summary_len,
        config.max_summary_len,
        char_len,
        image_count,
        data.clone(),
    ).await?;

    let mut hasher = Sha3_256::new();
    hasher.update(data.as_bytes());
    hasher.update(title.as_bytes());
    hasher.update(summary.as_bytes());
    let uid = hasher.finalize();

    Ok(Chunk {
        data,
        images,
        char_len,
        image_count,
        title,
        summary,
        file: normalize(&file)?,
        index: file_index,
        uid: Uid::from_str(&format!("{:064x}", uid))?,
        build_info,
        timestamp: 0, // Placeholder, needs to be set correctly
        searchable: true, // Placeholder, needs to be set correctly
        muse_summaries: HashMap::new(), // Placeholder
        source: ChunkSource::default(), // Placeholder
    })
}
