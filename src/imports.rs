use async_recursion::async_recursion;
use chrono::{Days, Local};
use crate::{
    AddMode,
    AgentAction,
    Audit,
    Error,
    IIStatus,
    Index,
    INDEX_DIR_NAME,
    Keywords,
    LoadMode,
    MODEL_FILE_NAME,
    MergeMode,
    ProcessedDoc,
    PullResult,
    PushResult,
    QueryTurn,
    RemoveResult,
    SummaryMode,
    UidQueryConfig,
    get_build_options,
    get_compatibility_warning,
    into_multi_modal_contents,
    ApiConfig,
};
use ragit_schema::{FileSchema, ImageSchema, Prettify};
use ragit_api::{Model, ModelRaw, get_model_by_name, Request, ModelQASystem, ModelQAResult, QualityScores};
use ragit_cli::{
    ArgCount,
    ArgParser,
    ArgType,
    Span,
    parse_pre_args,
};
use ragit_fs::{
    basename,
    create_dir,
    exists,
    join,
    join3,
    read_dir,
    read_string,
    set_current_dir,
};
use ragit_pdl::{Pdl, encode_base64, parse_schema, render_pdl_schema, Message, Role};
use serde_json::{Map, Value};
use std::env;
use std::io::Write;
use ragit_utils::string_utils::get_closest_string;
