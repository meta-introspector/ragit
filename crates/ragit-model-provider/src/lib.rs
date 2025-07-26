use ragit_error::ApiError as Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TestModel {
    Dummy,
    Stdin,
    Error,
}

use ragit_types::pdl_types::Message;

impl TestModel {
pub fn get_dummy_response(&self, _messages: &[Message]) -> Result<String, Error> {
        match self {
            TestModel::Dummy => Ok(String::from("dummy response from TestModel::Dummy")),
            TestModel::Stdin => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                Ok(input.trim().to_string())
            },
            TestModel::Error => Err(Error::TestModelError),
        }
    }
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
                ModelProvider::Test(TestModel::Dummy) => "test_dummy",
                ModelProvider::Test(TestModel::Stdin) => "test_stdin",
                ModelProvider::Test(TestModel::Error) => "test_error",
            },
        )
    }
}
