use async_trait::async_trait;

use crate::{requests::api_manager::ApiManager, RobberError};
use serde::Deserialize;

use super::users::{User, UserInteraction};

#[derive(Deserialize)]
pub struct GetMembersResponse {
    count: i32,
    items: Vec<i32>,
}

#[derive(Deserialize)]
pub struct GetMembers {
    response: Option<GetMembersResponse>,
}

impl GetMembers {
    fn validate(&self) -> bool {
        self.response.is_some()
    }
}

#[async_trait]
pub trait GroupInteraction {
    async fn get_members_ids(&self, group_id: i32) -> Result<Vec<i32>, RobberError>;
    async fn get_members(&self, group_id: i32, fields: &str) -> Result<Vec<User>, RobberError>;
}

#[async_trait]
impl GroupInteraction for ApiManager {
    async fn get_members_ids(&self, group_id: i32) -> Result<Vec<i32>, RobberError> {
        let spy_request = self
            .request_json::<_, GetMembers>("groups.getMembers", &[("group_id", group_id)])
            .await?;

        if !spy_request.validate() {
            return Err(RobberError::APIError);
        }

        let resp = spy_request.response.unwrap();
        let mut result: Vec<i32> = Vec::with_capacity(resp.count as usize);

        for i in 0..=(resp.count / 1000) {
            let request = self
                .request_json::<_, GetMembers>(
                    "groups.getMembers",
                    &[("group_id", group_id), ("offset", i * 1000)],
                )
                .await?;
            if !request.validate() {
                return Err(RobberError::APIError);
            }
            let mut resp = request.response.unwrap();

            result.extend(resp.items.drain(..));

            tokio::time::sleep(tokio::time::Duration::from_millis(350)).await;
        }

        Ok(result)
    }
    async fn get_members(&self, group_id: i32, fields: &str) -> Result<Vec<User>, RobberError> {
        let ids = self.get_members_ids(group_id).await?;
        let ids: Vec<String> = ids.into_iter().map(|x| x.to_string()).collect();
        let chunks = ids.chunks(100);

        let mut result = Vec::with_capacity(ids.len());

        for chunk in chunks {
            let ids_list = chunk.join(", ");
            let users = self.get_users(&ids_list, fields).await;

            if let Ok(mut users) = users {
                result.extend(users.drain(..));
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
        }

        Ok(result)
    }
}