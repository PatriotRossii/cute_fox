#![feature(async_closure)]
#![feature(exact_size_is_empty)]

use async_trait::async_trait;
use stages::{
    groups::GroupInteraction,
    users::{User, UserInteraction},
};
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
    GetMembers { group_id: i32, fields: String },
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
            CuteTask::GetMembers { group_id, fields } => {
                let spy_manager = &self.managers[0];
                let user_ids = spy_manager.get_members_ids(group_id).await?;

                self.execute(CuteTask::GetUsers { user_ids, fields }).await
            }
            CuteTask::GetUsers { user_ids, fields } => {
                let mut result = Vec::new();
                let mut chunks = user_ids.chunks(1000);

                let fields = Arc::new(fields);
                let mut tasks = Vec::new();

                'inner: while !chunks.is_empty() {
                    for manager in &*self.managers {
                        let chunk = match chunks.next() {
                            Some(e) => e.to_owned(),
                            None => break 'inner,
                        };
                        let new_manager = manager.clone();
                        let fields = fields.clone();

                        tasks.push(tokio::spawn(async move {
                            let our_chunk = chunk;
                            new_manager.get_users_unchecked(&our_chunk, fields.as_ref()).await
                        }));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                }

                for task in tasks {
                    result.extend(task.await.map_err(RobberError::JoinError)??.drain(..));
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
