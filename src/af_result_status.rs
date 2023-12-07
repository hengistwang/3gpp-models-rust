use serde::{Deserialize, Serialize};

/// AfResultStatus : Possible values are: - SUCCESS: The application layer is ready or the relocation is completed. - TEMPORARY_CONGESTION: The application relocation fails due to temporary congestion. - RELOC_NO_ALLOWED: The application relocation fails because application relocation is not allowed. - OTHER: The application relocation fails due to other reason.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfResultStatus {}

impl AfResultStatus {
    /// Possible values are: - SUCCESS: The application layer is ready or the relocation is completed. - TEMPORARY_CONGESTION: The application relocation fails due to temporary congestion. - RELOC_NO_ALLOWED: The application relocation fails because application relocation is not allowed. - OTHER: The application relocation fails due to other reason.
    pub fn new() -> AfResultStatus {
        AfResultStatus {}
    }
}
