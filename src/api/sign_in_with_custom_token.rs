use serde::{Deserialize, Serialize};
use crate::api::FailResponse;
use crate::Error;

impl crate::FireAuth {
    pub async fn sign_in_custom_token(&self, token: &str) -> Result<Response, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithCustomToken?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(&url)
            .header("Content-Type", "application/json")
            .json(&SignInPayload {
                token,
                // According to the docs, this should always be true.
                return_secure_token: true,
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::SignIn(error.message));
        }

        let body = resp.json::<Response>().await?;
        Ok(body)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignInPayload<'a> {
    token: &'a str,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id_token: String,
    pub refresh_token: String,
    pub expires_in: String,
}
