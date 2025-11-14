use ragit_types::ApiError;
use ragit_pdl::Error as PdlError;

pub struct WrappedPdlError(pub PdlError);

impl From<WrappedPdlError> for ApiError {
    fn from(err: WrappedPdlError) -> Self {
        ApiError::Internal(format!("PDL error: {}", err.0))
    }
}
