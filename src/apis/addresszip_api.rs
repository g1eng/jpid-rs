use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1AddresszipPostError {
    Status400(models::BadRequest),
    Status401(models::Unauthorized),
    Status404(models::NotFound),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

pub async fn api_v1_addresszip_post(
    configuration: &configuration::Configuration,
    address_req: models::AddressReq,
    ec_uid: Option<&str>,
) -> Result<models::AddressRes, Error<ApiV1AddresszipPostError>> {
    let p_address_req = address_req;
    let p_ec_uid = ec_uid;

    let uri_str = format!("{}/api/v1/addresszip", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_ec_uid {
        req_builder = req_builder.query(&[("ec_uid", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_address_req);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddressRes`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddressRes`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV1AddresszipPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
