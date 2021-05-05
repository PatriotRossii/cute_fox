#![feature(async_closure)]
#![feature(exact_size_is_empty)]

use async_trait::async_trait;
use stages::users::UserInteraction;
use std::sync::Arc;
use tokio::task;

use requests::api_manager::ApiManager;

pub mod requests;
pub mod stages;

#[derive(Debug)]
pub enum RobberError {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
    APIError,
}

#[derive(Debug, Clone)]
pub enum CuteTask {
    GetMembers { group_id: i32 },
    GetUsers { user_ids: Vec<i32> },
}

#[async_trait]
pub trait CuteExecutor {
    async fn execute(&self, task: CuteTask);
}

#[async_trait]
impl CuteExecutor for CuteFox {
    async fn execute(&self, task: CuteTask) {
        match task {
            CuteTask::GetMembers { group_id } => {}
            CuteTask::GetUsers { user_ids } => {
                let mut tasks = Vec::new();
                let mut chunks = user_ids.chunks(100);

                let mut result = Vec::new();

                while !chunks.is_empty() {
                    for manager in &*self.managers {
                        if let Some(e) = chunks.next() {
                            tasks.push(manager.get_users(e, ""));
                        } else {
                            break;
                        }
                    }
                }

                for task in tasks {
                    result.extend(task.await.unwrap());
                }
            }
        }
    }
}

pub struct CuteFox {
    count_of_managers: usize,
    managers: Arc<Vec<ApiManager>>,
}

impl CuteFox {
    pub fn new(tokens: &[&str], api_version: &str) -> Self {
        let managers: Vec<ApiManager> = tokens
            .iter()
            .map(|&e| ApiManager::new(e, api_version))
            .collect();

        Self {
            count_of_managers: managers.len(),
            managers: Arc::new(managers),
        }
    }
}
