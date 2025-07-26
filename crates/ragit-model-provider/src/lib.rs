use ragit_error::ApiError as Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct TestModel {
    name: String
}
#[derive(Clone, Debug, PartialEq)]
pub enum ModelProvider {
    OpenAi {
        url: String,
    },
    Cohere,
    Anthropic,
    Google,

    /// for test
    /// 1. doesn't require api key
    /// 2. needs no network
    Test(TestModel),
}

impl ModelProvider {
    pub fn parse_chat_response(&self, s: &str) -> Result<serde_json::Value, Error> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn parse(s: &str, url: &Option<String>) -> Result<Self, Error> {
        match s
            .to_ascii_lowercase()
            .replace(" ", "")
            .replace("-", "")
            .as_str()
        {
            "openai" => match url {
                Some(url) => Ok(ModelProvider::OpenAi {
                    url: url.to_string(),
                }),
                None => Ok(ModelProvider::OpenAi {
                    url: String::from("https://api.openai.com/v1/chat/completions"),
                }),
            },
            "cohere" => Ok(ModelProvider::Cohere),
            "anthropic" => Ok(ModelProvider::Anthropic),
            "google" => Ok(ModelProvider::Google),
            _ => Err(Error::InvalidApiProvider(s.to_string())),
        }
    }
}

impl fmt::Display for ModelProvider {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                ModelProvider::OpenAi { .. } => "openai",
                ModelProvider::Cohere => "cohere",
                ModelProvider::Anthropic => "anthropic",
                ModelProvider::Google => "google",
                ModelProvider::Test(_) => "test",
            },
        )
    }
}
