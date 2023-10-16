use axios::{error::AxiosError, Axios, AxiosRequestConfig};
use serde_json::Value;

#[tauri::command]
pub async fn request(data: Value) -> Result<Value, String> {
  let url = data
    .get("url")
    .and_then(Value::as_str)
    .unwrap_or_default()
    .to_string();

  let method = data
    .get("method")
    .and_then(Value::as_str)
    .unwrap_or_default()
    .to_string();

  let body = data.get("body");

  let api = Axios::default();

  let res = api
    .request::<Value, Value, Value>(AxiosRequestConfig { url, method }, body)
    .await;

  let result_data = match res {
    Ok(res) => res,
    Err(err) => match err {
      AxiosError::MethodError(err) => err,
      _ => Value::Null,
    },
  };

  Ok(result_data)
}
