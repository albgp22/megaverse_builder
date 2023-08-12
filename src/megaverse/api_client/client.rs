use crate::megaverse::api_client::types::GoalResponse;
use crate::megaverse::astral::objects::AstralObject;
use crate::megaverse::config::handler::Config;
use log;
use log::{info, warn};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use std::{error, thread};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct ApiClient {
    pub(super) cfg: Config,
    pub(super) client: Client,
    pub(super) retries: u32,
}

impl ApiClient {
    fn post(&self, endpoint: String, body: String) -> Result<(StatusCode, String)> {
        for attempt in 0..=self.retries {
            match self
                .client
                .post(endpoint.clone())
                .body(body.clone())
                .header("Content-type", "application/json")
                .send()
            {
                Ok(res) if res.status()==200 => {
                    return Ok((res.status(), res.text()?))
                }
                Err(e) => {
                    if attempt == self.retries {
                        return Err(Box::new(e));
                    } else {
                        warn!("Got error {e:?}, retrying.");
                    }
                }
                Ok(res) => {
                    warn!("Got status code {sc}!=200. Retrying", sc=res.status());
                    if res.status()==429{
                        warn!("Too many requests. Sleeping for 10s.");
                        thread::sleep(Duration::from_secs(10));
                    }
                }
            }
        }
        unreachable!()
    }

    pub fn create_object(&self, obj: &AstralObject) -> Result<()> {
        let endpoint = self.get_resource_endpoint(obj);
        let request_body = obj.json_with_additional_fields(HashMap::from([(
            "candidateId".to_string(),
            self.cfg.candidate_id.to_string(),
        )]));
        info!("Creating object {obj:?} in API {endpoint} with json body {request_body}");
        let (status, body) = self.post(endpoint, request_body)?;
        info!("Response code: {status}, Response body: {body}");
        Ok(())
    }

    fn get_resource_endpoint(&self, obj: &AstralObject) -> String {
        format!(
            "{protocol}://{hostname}{api_endpoint}{resource}",
            protocol = self.cfg.protocol.to_string(),
            hostname = self.cfg.host,
            api_endpoint = self.cfg.api_endpoint,
            resource = match obj {
                AstralObject::Cometh { .. } => self.cfg.resources.comeths.clone(),
                AstralObject::Polyanet { .. } => self.cfg.resources.polyanets.clone(),
                AstralObject::Soloon { .. } => self.cfg.resources.soloons.clone(),
            }
        )
    }

    fn goal_endpoint(&self) -> String {
        format!(
            "{protocol}://{hostname}{api_endpoint}{resource}",
            protocol = self.cfg.protocol.to_string(),
            hostname = self.cfg.host,
            api_endpoint = self.cfg.api_endpoint,
            resource = self.cfg.resources.goal
        )
    }

    pub fn get_goal_resp(&self) -> Result<GoalResponse> {
        let endpoint = self.goal_endpoint();
        let res = self.client.get(endpoint).send()?;
        let map: GoalResponse = res.json()?;
        Ok(map)
    }

    #[allow(dead_code)]
    pub fn get_goal_dims(&self) -> Result<(u32, u32)> {
        let goal_resp = self.get_goal_resp()?;
        Ok((goal_resp.goal.len() as u32, goal_resp.goal[0].len() as u32))
    }
}
