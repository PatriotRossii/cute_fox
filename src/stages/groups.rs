use async_trait::async_trait;

use crate::{
    requests::api_manager::{ApiManager, API_TIMEOUT_MS},
    RobberError,
};
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

            tokio::time::sleep(tokio::time::Duration::from_millis(API_TIMEOUT_MS)).await;
        }

        Ok(result)
    }
    async fn get_members(&self, group_id: i32, fields: &str) -> Result<Vec<User>, RobberError> {
        let ids = self.get_members_ids(group_id).await?;
        Ok(self.get_users(&ids, fields).await?)
    }
}
