use anyhow::Result;
use reqwest::{blocking::Response, header};

pub fn call_url_with_headers(url: String, headers: header::HeaderMap) -> Result<Response> {
    Ok(reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?
        .get(url)
        .send()?)
}

pub fn create_cookie_header(cookie: &String) -> Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&format!("session={}", cookie))?;
    auth_value.set_sensitive(true);
    headers.insert("Cookie", auth_value);
    Ok(headers)
}
