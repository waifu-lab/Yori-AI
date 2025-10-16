use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontendError {

}

#[derive(Error, Debug)]
pub enum BackendError {
    // Path errors
    #[error("Invalid path")]
    InvalidPath,

    // IO errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    // Serialization errors
    #[error("Serialization failed: {0}")]
    SerializationFailed(bincode::Error),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(bincode::Error),

    // Project errors
    #[error("Project not found")]
    ProjectNotFound,

    // Plugin errors
    #[error(transparent)]
    PluginLoadFailed(#[from] libloading::Error),
    #[error("Invalid plugin format")]
    InvalidPluginFormat,
}