#![feature(async_closure)]
#![feature(exact_size_is_empty)]

use async_trait::async_trait;
use stages::users::{User, UserInteraction};
use std::sync::Arc;
use tokio::task::JoinError;

use requests::api_manager::ApiManager;

pub mod requests;
pub mod stages;

#[derive(Debug)]
pub enum RobberError {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
    JoinError(JoinError),
    APIError,
}

#[derive(Debug, Clone)]
pub enum CuteTask {
    GetMembers { group_id: i32 },
    GetUsers { user_ids: Vec<i32>, fields: String },
}

#[derive(Debug)]
pub enum CuteValue {
    Users(Vec<User>),
}

#[async_trait]
pub trait CuteExecutor {
    async fn execute(&self, task: CuteTask) -> Result<CuteValue, RobberError>;
}

#[async_trait]
impl CuteExecutor for CuteFox {
    async fn execute(&self, task: CuteTask) -> Result<CuteValue, RobberError> {
        match task {
            CuteTask::GetMembers { group_id: _ } => {
                unimplemented!()
            }
            CuteTask::GetUsers { user_ids, fields } => {
                let mut result = Vec::new();
                let mut chunks = user_ids.chunks(1000);

                let fields = Arc::new(fields);

                while !chunks.is_empty() {
                    let mut tasks = Vec::new();

                    for manager in &*self.managers {
                        let chunk = match chunks.next() {
                            Some(e) => e.to_owned(),
                            None => break,
                        };
                        let new_manager = manager.clone();
                        let fields = fields.clone();

                        tasks.push(tokio::spawn(async move {
                            let our_chunk = chunk;
                            new_manager.get_users(&our_chunk, fields.as_ref()).await
                        }));
                    }

                    for task in tasks {
                        let mut users = task.await.map_err(RobberError::JoinError)??;
                        result.extend(users.drain(..));
                    }
                }

                Ok(CuteValue::Users(result))
            }
        }
    }
}

pub struct CuteFox {
    managers: Arc<Vec<Arc<ApiManager>>>,
}

impl CuteFox {
    pub fn new(tokens: &[String], api_version: &str) -> Self {
        let managers: Vec<Arc<ApiManager>> = tokens
            .iter()
            .map(|e| Arc::new(ApiManager::new(e, api_version)))
            .collect();

        Self {
            managers: Arc::new(managers),
        }
    }
}
