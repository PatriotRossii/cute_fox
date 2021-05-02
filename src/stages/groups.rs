use std::usize;

use crate::{requests::api_manager::ApiManager, RobberError};
use serde::Deserialize;

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

pub struct Group {}

impl Group {
    pub async fn get_members_ids(api: &ApiManager, group_id: i32) -> Result<Vec<i32>, RobberError> {
        let spy_request = api
            .request_json::<_, GetMembers>("groups.getMembers", &[("group_id", group_id)])
            .await?;

        if !spy_request.validate() {
            return Err(RobberError::APIError);
        }

        let resp = spy_request.response.unwrap();
        let mut result: Vec<i32> = Vec::with_capacity(resp.count as usize);

        for i in 0..=(resp.count / 1000) {
            let request = api
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
}
