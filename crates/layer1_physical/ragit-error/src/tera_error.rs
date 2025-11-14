use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeraError {
    #[error("dummy tera error")]
    Dummy,
}
