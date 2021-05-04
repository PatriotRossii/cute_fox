use std::future::Future;

use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::RobberError;

pub const API_VERSION: &str = "5.130";

pub struct ApiManager {
    token: String,
    version: String,
    client: Client,
}

impl ApiManager {
    const API_SERVER: &'static str = "https://api.vk.com/method";

    pub fn new<T1, T2>(token: T1, version: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        Self {
            token: token.into(),
            version: version.into(),
            client: Client::new(),
        }
    }

    pub fn request<T: Serialize + ?Sized>(
        &self,
        method: &str,
        params: &T,
    ) -> impl Future<Output = Result<Response, Error>> {
        let request = self
            .client
            .get(format!("{}/{}", ApiManager::API_SERVER, method));

        let request = request.query(params);
        let request = request.query(&[("access_token", &self.token), ("v", &self.version)]);

        request.send()
    }

    pub async fn request_json<'a, T: Serialize + ?Sized, Y>(
        &self,
        method: &str,
        params: &T,
    ) -> Result<Y, RobberError>
    where
        Y: for<'de> Deserialize<'de>,
    {
        Ok(self
            .request(method, params)
            .await
            .map_err(RobberError::ReqwestError)?
            .json::<Y>()
            .await
            .map_err(RobberError::ReqwestError)?)
    }
}
