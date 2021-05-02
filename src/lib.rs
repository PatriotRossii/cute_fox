pub mod requests;
pub mod stages;

#[derive(Debug)]
pub enum RobberError {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
    APIError,
}
