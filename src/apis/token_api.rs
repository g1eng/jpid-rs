use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PosttokenError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status403(models::Forbidden),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

pub async fn posttoken(
    configuration: &configuration::Configuration,
    x_forwarded_for: &str,
    jtoken_req: models::JtokenReq,
) -> Result<models::JtokenRes, Error<PosttokenError>> {
    let p_x_forwarded_for = x_forwarded_for;
    let p_jtoken_req = jtoken_req;

    let uri_str = format!("{}/api/v1/j/token", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("x-forwarded-for", p_x_forwarded_for.to_string());
    req_builder = req_builder.json(&p_jtoken_req);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::JtokenRes`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::JtokenRes`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PosttokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
