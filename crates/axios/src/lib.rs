use request::string_to_method;
use reqwest::Client;
use serde::Serialize;

use crate::error::AxiosError;

pub mod error;
pub mod request;

#[derive(Debug, Default)]
pub enum HttpMethod {
  #[default]
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
  CONNECT,
  TRACE,
}

pub struct AxiosConfig {
  pub base_url: String,
  pub timeout: u32,
  pub headers: Vec<String>,
}

impl Default for AxiosConfig {
  fn default() -> Self {
    Self {
      base_url: String::from(""),
      timeout: 10000,
      headers: Vec::new(),
    }
  }
}

impl AxiosConfig {
  pub fn new() -> Self {
    Self::default()
  }
}

pub struct Axios {
  client: Client,
  pub config: AxiosConfig,
}

impl From<AxiosConfig> for Axios {
  fn from(config: AxiosConfig) -> Self {
    Self {
      config,
      ..Default::default()
    }
  }
}

#[derive(Default)]
pub struct AxiosRequestConfig {
  pub url: String,
  pub method: String,
  // pub params: String,
  // pub headers: Vec<String>,
  // pub timeout: u32,
}

impl Axios {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn request<TReq, TRes, TErr>(
    &self,
    cfg: AxiosRequestConfig,
    data: Option<&TReq>,
  ) -> Result<TRes, AxiosError<TErr>>
  where
    TReq: Serialize,
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    let full_url = format!("{}{}", self.config.base_url, cfg.url);

    let req_builder = self
      .client
      .request(string_to_method(cfg.method), full_url)
      .header("Content-Type", "application/json")
      .header("User-Agent", "hivexdev");

    let req_builder = if let Some(data) = data {
      req_builder.form(data)
    } else {
      req_builder
    };

    let res = req_builder.send().await?;

    let status = res.status();

    let body = res.text().await?;

    if status.is_success() {
      let response: TRes = serde_json::from_str(&body)?;
      Ok(response)
    } else {
      let error_response: TErr = serde_json::from_str(&body)?;
      Err(AxiosError::MethodError(error_response))
    }
  }
}

impl Default for Axios {
  fn default() -> Self {
    Self {
      client: Client::new(),
      config: AxiosConfig::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde_json::Value;

  use super::*;

  #[derive(Debug, Serialize, Deserialize)]
  struct RequestPayload {}

  #[derive(Debug, Serialize, Deserialize)]
  struct ResponsePayload {
    pub login: String,
    pub id: u32,
    pub created_at: String,
  }

  #[tokio::test]
  async fn test_new() {
    let api = Axios::from(AxiosConfig {
      base_url: String::from("https://api.github.com"),
      ..Default::default()
    });

    let res = api
      .request::<Value, Value, Value>(
        AxiosRequestConfig {
          url: String::from("/users/hivexdev"),
          method: String::from("GET"),
        },
        None,
      )
      .await;

    match res {
      Ok(res) => println!("{:?}", res),
      Err(err) => match err {
        AxiosError::MethodError(err) => println!("{:?}", err),
        _ => println!("pau {:?}", err),
      },
    }
  }
}
