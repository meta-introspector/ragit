use crate::Error;
use chrono::{DateTime, Datelike, Local, Utc};
use ragit_fs::{WriteMode, create_dir_all, exists, parent, read_string, write_string};
use ragit_types::{JsonType, pdl_types::Message};
use ragit_types::AuditRecordAt;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::path::Path;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuditRecord {
    pub input_tokens: u64,
    pub output_tokens: u64,

    // Divide this by 1 million to get dollars
    pub input_cost: u64,
    pub output_cost: u64,
}

impl AddAssign<AuditRecord> for AuditRecord {
    fn add_assign(&mut self, rhs: AuditRecord) {
        self.input_tokens += rhs.input_tokens;
        self.output_tokens += rhs.output_tokens;
        self.input_cost += rhs.input_cost;
        self.output_cost += rhs.output_cost;
    }
}

impl Default for AuditRecord {
    fn default() -> Self {
        Self {
            input_tokens: 0,
            output_tokens: 0,
            input_cost: 0,
            output_cost: 0,
        }
    }
}

impl From<&AuditRecord> for Value {
    fn from(r: &AuditRecord) -> Value {
        Value::Array(vec![
            Value::from(r.input_tokens),
            Value::from(r.output_tokens),
            Value::from(r.input_cost),
            Value::from(r.output_cost),
        ])
    }
}

impl TryFrom<&Value> for AuditRecord {
    type Error = Error;

    fn try_from(j: &Value) -> Result<AuditRecord, Error> {
        let mut result = vec![];

        match &j {
            Value::Array(arr) => {
                if arr.len() != 4 {
                    return Err(Error::WrongSchema(format!(
                        "expected an array of length 4, but got length {}",
                        arr.len()
                    )));
                }

                for r in arr.iter() {
                    match r.as_u64() {
                        Some(n) => {
                            result.push(n);
                        }
                        None => {
                            return Err(Error::JsonTypeError {
                                expected: JsonType::U64,
                                got: r.into(),
                            });
                        }
                    }
                }

                Ok(AuditRecord {
                    input_tokens: result[0],
                    output_tokens: result[1],
                    input_cost: result[2],
                    output_cost: result[3],
                })
            }
            _ => Err(Error::JsonTypeError {
                expected: JsonType::Array,
                got: j.into(),
            }),
        }
    }
}

fn records_from_json(j: &Value) -> Result<HashMap<String, AuditRecord>, Error> {
    match j {
        Value::Object(obj) => {
            let mut result = HashMap::with_capacity(obj.len());

            for (key, value) in obj.iter() {
                result.insert(key.to_string(), AuditRecord::try_from(value)?);
            }

            Ok(result)
        }
        Value::Array(arr) => {
            let mut result: HashMap<String, AuditRecord> = HashMap::new();

            for r in arr.iter() {
                let AuditRecordLegacy {
                    time,
                    input,
                    output,
                    input_weight,
                    output_weight,
                } = AuditRecordLegacy::try_from(r)?;
                // NOTE: RecordLegacy -> Record conversion might introduce a few hours of errors.
                let date = match DateTime::<Utc>::from_timestamp(time as i64, 0) {
                    Some(date) => format!("{:04}{:02}{:02}", date.year(), date.month(), date.day()),
                    None => format!("19700101"),
                };
                let new_record = AuditRecord {
                    input_tokens: input,
                    output_tokens: output,
                    input_cost: input * input_weight / 1000,
                    output_cost: output * output_weight / 1000,
                };

                match result.get_mut(&date) {
                    Some(record) => {
                        *record += new_record;
                    }
                    None => {
                        result.insert(date, new_record);
                    }
                }
            }

            Ok(result)
        }
        _ => Err(Error::JsonTypeError {
            expected: JsonType::Object,
            got: j.into(),
        }),
    }
}

#[derive(Clone)]
pub struct Tracker(pub HashMap<String, HashMap<String, AuditRecord>>); // user_name -> usage

use std::sync::Arc;

fn map_serde_json_error<T>(result: Result<T, serde_json::Error>) -> Result<T, Error> {
    result.map_err(|e| Error::JsonSerdeError(Arc::new(e)))
}

impl Tracker {
    pub fn new() -> Self {
        Tracker(HashMap::new())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        let content = read_string(path).map_err(Error::FileError)?;
        let j: Value = serde_json::from_str(&content).map_err(Error::JsonSerdeError)?;
        Tracker::try_from(&j)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        Ok(write_string(
            path,
            &map_serde_json_error(serde_json::to_string_pretty(&Value::from(self)))?,
            WriteMode::Atomic,
        ).map_err(Error::FileError)?)
    }
}

impl TryFrom<&Value> for Tracker {
    type Error = Error;

    fn try_from(v: &Value) -> Result<Tracker, Error> {
        match v {
            Value::Object(obj) => {
                let mut result = HashMap::new();

                for (k, v) in obj.iter() {
                    result.insert(k.to_string(), records_from_json(v)?);
                }

                Ok(Tracker(result))
            }
            _ => Err(Error::JsonTypeError {
                expected: JsonType::Object,
                got: v.into(),
            }),
        }
    }
}

impl From<&Tracker> for Value {
    fn from(t: &Tracker) -> Value {
        Value::Object(
            t.0.iter()
                .map(|(id, records)| {
                    (
                        id.to_string(),
                        Value::Object(
                            records
                                .iter()
                                .map(|(date, record)| (date.to_string(), Value::from(record)))
                                .collect::<Map<_, _>>(),
                        ),
                    )
                })
                .collect(),
        )
    }
}

pub fn dump_api_usage(
    at: &AuditRecordAt,
    input_tokens: u64,
    output_tokens: u64,

    // dollars per 1 billion tokens
    input_weight: u64,
    output_weight: u64,

    // legacy option
    _clean_up_records: bool,
) -> Result<(), Error> {
    let mut tracker = Tracker::load_from_file(&at.path)?;
    let today = Local::now();
    let today = format!("{:04}{:02}{:02}", today.year(), today.month(), today.day());
    let new_record = AuditRecord {
        input_tokens,
        output_tokens,
        input_cost: input_tokens * input_weight / 1000,
        output_cost: output_tokens * output_weight / 1000,
    };

    match tracker.0.get_mut(&at.id) {
        Some(records) => match records.get_mut(&today) {
            Some(record) => {
                *record += new_record;
            }
            None => {
                records.insert(today, new_record);
            }
        },
        None => {
            tracker
                .0
                .insert(at.id.clone(), [(today, new_record)].into_iter().collect());
        }
    }

    tracker.save_to_file(&at.path)?;
    Ok(())
}

pub fn get_user_usage_data_since(
    at: AuditRecordAt,
    since: DateTime<Local>,
) -> Option<HashMap<String, AuditRecord>> {
    let since = format!("{:04}{:02}{:02}", since.year(), since.month(), since.day());

    match Tracker::load_from_file(&at.path) {
        Ok(tracker) => match tracker.0.get(&at.id) {
            Some(records) => Some(
                records
                    .iter()
                    .filter(|(date, _)| date >= &&since)
                    .map(|(date, record)| (date.to_string(), record.clone()))
                    .collect(),
            ),
            None => None,
        },
        _ => None,
    }
}

pub fn get_usage_data_since(
    path: &str,
    since: DateTime<Local>,
) -> Option<HashMap<String, AuditRecord>> {
    let since = format!("{:04}{:02}{:02}", since.year(), since.month(), since.day());

    match Tracker::load_from_file(path) {
        Ok(tracker) => {
            let mut result = HashMap::new();

            for records in tracker.0.values() {
                for (date, record) in records.iter() {
                    if date >= &since {
                        result.insert(date.to_string(), record.clone());
                    }
                }
            }

            Some(result)
        }
        _ => None,
    }
}

/// It returns the cost in dollars (in a formatted string), without any currency unit.
pub fn calc_usage(records: &HashMap<String, AuditRecord>) -> String {
    // cost * 1M
    let mut total: u64 = records
        .values()
        .map(
            |AuditRecord {
                 input_cost,
                 output_cost,
                 ..
             }| *input_cost + *output_cost,
        )
        .sum();

    // cost * 1K
    total /= 1000;

    format!("{:.3}", total as f64 / 1_000.0)
}

pub fn dump_pdl(
    messages: &[Message],
    response: &str,
    reasoning: &Option<String>,
    path: &str,
    metadata: String,
) -> Result<(), Error> {
    let mut markdown = vec![];

    for message in messages.iter() {
        markdown.push(format!(
            "\n\n<|{:?}|>\n\n{}",
            message.role,
            message
                .content
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""),
        ));
    }

    markdown.push(format!(
        "\n\n<|Assistant|>{}\n\n{response}",
        if let Some(reasoning) = reasoning {
            format!("\n\n<|Reasoning|>\n\n{reasoning}\n\n")
        } else {
            String::new()
        },
    ));
    markdown.push(format!("{}# {metadata} #{}", '{', '}')); // tera format

    if let Ok(parent_path_buf) = parent(Path::new(path)) {
        if !exists(&parent_path_buf) {
            create_dir_all(parent_path_buf.to_str().unwrap()).map_err(Error::FileError)?;
        }
    }

    write_string(path, &markdown.join("\n"), WriteMode::CreateOrTruncate).map_err(Error::FileError)?;

    Ok(())
}

/*
 * Below is a previous implementation of `AuditRecord`.
 * I found it painfully slowing, so I rewrite it from scratch (above).
 */

impl From<AuditRecordLegacy> for Value {
    fn from(r: AuditRecordLegacy) -> Value {
        Value::Array(vec![
            Value::from(r.time),
            Value::from(r.input),
            Value::from(r.output),
            Value::from(r.input_weight),
            Value::from(r.output_weight),
        ])
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuditRecordLegacy {
    pub time: u64,
    pub input: u64,
    pub output: u64,

    // dollars per 1 billion tokens
    pub input_weight: u64,
    pub output_weight: u64,
}

impl TryFrom<&Value> for AuditRecordLegacy {
    type Error = Error;

    fn try_from(j: &Value) -> Result<AuditRecordLegacy, Error> {
        let mut result = vec![];

        match &j {
            Value::Array(arr) => {
                if arr.len() != 5 {
                    return Err(Error::WrongSchema(format!(
                        "expected an array of length 5, but got length {}",
                        arr.len()
                    )));
                }

                for r in arr.iter() {
                    match r.as_u64() {
                        Some(n) => {
                            result.push(n);
                        }
                        None => {
                            return Err(Error::JsonTypeError {
                                expected: JsonType::U64,
                                got: r.into(),
                            });
                        }
                    }
                }

                Ok(AuditRecordLegacy {
                    time: result[0],
                    input: result[1],
                    output: result[2],
                    input_weight: result[3],
                    output_weight: result[4],
                })
            }
            _ => Err(Error::JsonTypeError {
                expected: JsonType::Array,
                got: j.into(),
            }),
        }
    }
}
