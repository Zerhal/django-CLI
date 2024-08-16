use thiserror::Error;


#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("User cancelled the operation")]
    UserCancelled,

    #[error("Failed to run command: {0}")]
    CommandFailed(String),

    #[error("Unsupported frontend framework: {0}")]
    UnsupportedFramework(String),

    #[error("Template rendering error: {0}")]
    TemplateError(#[from] tera::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error), // Added this variant

    // Add more error types as needed
}