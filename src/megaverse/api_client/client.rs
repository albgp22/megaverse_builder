use crate::megaverse::api_client::types::GoalResponse;
use crate::megaverse::astral::objects::AstralObject;
use crate::megaverse::config::handler::Config;
use crate::megaverse::procedural_macros::retry::exponential_backoff;
use log;
use log::{info};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use std::{error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct ApiClient {
    pub(super) cfg: Config,
    pub(super) client: Client,
    pub(super) retries: u32,
}

impl ApiClient {
    fn post(&self, endpoint: &str, body: &str) -> Result<(StatusCode, String)> {
        match self
            .client
            .post(endpoint.to_string())
            .body(body.to_string())
            .header("Content-type", "application/json")
            .send()
        {
            Ok(res) if res.status() == 200 => Ok((res.status(), res.text()?)),
            Err(e) => Err(Box::new(e)),
            Ok(res) => Err(format!("Wrong status code: {sc}", sc = res.status()).into()),
        }
    }

    // Create the astral object
    pub fn create_object(&self, obj: &AstralObject) -> Result<()> {
        let endpoint = self.get_resource_endpoint(obj);
        let request_body = obj.json_with_additional_fields(HashMap::from([(
            "candidateId".to_string(),
            self.cfg.candidate_id.to_string(),
        )]));

        info!("Creating object {obj:?} in API {endpoint} with json body {request_body}");
        // We can safely retry as, in this particular case, POST is idempotent.
        let post_function = move || self.post(&endpoint, &request_body);
        let (status, body) =
            exponential_backoff(self.retries, Duration::from_secs(1), 2, &post_function)?;
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

    /* Functions to retrieve and parse the goal map */
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
        let res = match self.client.get(endpoint).send() {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("Failed while getting the goal map from the API: {e}").into())
            }
        };
        let map: GoalResponse = match res.json() {
            Ok(map) => map,
            Err(e) => return Err(format!("Failed while parsing the goal map: {e}").into()),
        };

        Ok(map)
    }

    #[allow(dead_code)]
    pub fn get_goal_dims(&self) -> Result<(u32, u32)> {
        let goal_resp = self.get_goal_resp()?;
        Ok((goal_resp.goal.len() as u32, goal_resp.goal[0].len() as u32))
    }
}
