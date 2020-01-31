use failure::Fail;

/// Result type for errors or values derived from Stations.
pub type StationResult<T> = Result<T, StationError>;

/// Errors enumeration to cover Station-specific errors.
#[derive(Fail, Debug, Clone, PartialEq)]
pub enum StationError {
    /// An error that occurs when attempting to act on a telemetry field that doesn't exist
    #[fail(display = "Requested telemetry field was not found")]
    TelemetryFieldMissing,
}
