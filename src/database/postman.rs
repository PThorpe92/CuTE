use super::db::SavedCommand;
use crate::request::curl::Curl;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize)]
pub struct PostmanCollection {
    pub info: Info,
    pub item: Vec<HashMap<String, Value>>,
}

impl From<PostmanCollection> for Vec<SavedCommand> {
    fn from(collection: PostmanCollection) -> Vec<SavedCommand> {
        let mut saved_commands = Vec::new();
        let mut curl_cmd = Curl::new();
        collection.item.iter().for_each(|item| {
            if let Some(request) = item.get("request") {
                if let Some(request) = request.as_str() {
                    // this means its a get request
                    curl_cmd.set_url(request);
                    curl_cmd.set_get_method();
                } else if let Some(request) = request.as_object() {
                    if let Some(method) = request.get("method") {
                        if let Some(method) = method.as_str() {
                            match method {
                                "GET" => {
                                    curl_cmd.set_get_method();
                                }
                                "POST" => {
                                    curl_cmd.set_post_method();
                                }
                                "PUT" => {
                                    curl_cmd.set_put_method();
                                }
                                "DELETE" => {
                                    curl_cmd.set_delete_method();
                                }
                                "PATCH" => {
                                    curl_cmd.set_patch_method();
                                }
                                "HEAD" => {
                                    curl_cmd.set_head_method();
                                }
                                _ => {
                                    curl_cmd.set_get_method();
                                }
                            }
                        }
                    }
                    if let Some(headers) = request.get("header") {
                        if let Some(headers) = headers.as_array() {
                            headers.iter().for_each(|hdr| {
                                if let Some(hdr) = hdr.as_object() {
                                    if let Some(key) = hdr.get("key") {
                                        if let Some(key) = key.as_str() {
                                            if let Some(value) = hdr.get("value") {
                                                if let Some(value) = value.as_str() {
                                                    curl_cmd.add_headers(&format!(
                                                        "{}: {}",
                                                        key, value
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                    if let Some(body) = request.get("body") {
                        if let Some(body) = body.as_object() {
                            if let Some(mode) = body.get("mode") {
                                if let Some(mode) = mode.as_str() {
                                    match mode {
                                        "formdata" => {
                                            if let Some(data) = body.get("formdata") {
                                                if let Some(data) = data.as_array() {
                                                    let mut form_data = Vec::new();
                                                    data.iter().for_each(|d| {
                                                        if let Some(d) = d.as_object() {
                                                            if let Some(key) = d.get("key") {
                                                                if let Some(key) = key.as_str() {
                                                                    if let Some(value) =
                                                                        d.get("value")
                                                                    {
                                                                        if let Some(value) =
                                                                            value.as_str()
                                                                        {
                                                                            form_data
                                                                                .push((key, value));
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    });
                                                    curl_cmd.set_request_body(
                                                        &serde_json::to_string(&form_data)
                                                            .unwrap_or_default(),
                                                    );
                                                }
                                            }
                                        }
                                        "urlencoded" => {
                                            if let Some(data) = body.get("urlencoded") {
                                                if let Some(data) = data.as_array() {
                                                    data.iter().for_each(|d| {
                                                        if let Some(d) = d.as_object() {
                                                            if let Some(key) = d.get("key") {
                                                                if let Some(key) = key.as_str() {
                                                                    if let Some(value) =
                                                                        d.get("value")
                                                                    {
                                                                        if let Some(value) =
                                                                            value.as_str()
                                                                        {
                                                                            curl_cmd.url_encode(
                                                                                &format!(
                                                                                    "{}={}",
                                                                                    key, value
                                                                                ),
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                        "raw" => {
                                            if let Some(data) = body.get("raw") {
                                                if let Some(data) = data.as_str() {
                                                    curl_cmd.set_request_body(data);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(url) = request.get("url") {
                    if let Some(url) = url.as_object() {
                        if let Some(url) = url.get("raw") {
                            if let Some(url) = url.as_str() {
                                curl_cmd.set_url(url);
                            }
                        }
                    }
                }
                if let Some(cookie) = request.get("cookie") {
                    if let Some(cookie) = cookie.as_array() {
                        cookie.iter().for_each(|ck| {
                            if let Some(ck) = ck.as_object() {
                                if let Some(key) = ck.get("key") {
                                    if let Some(key) = key.as_str() {
                                        if let Some(value) = ck.get("value") {
                                            if let Some(value) = value.as_str() {
                                                curl_cmd.add_cookie(&format!("{}: {}", key, value));
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
            }
        });
        let cmd = curl_cmd.get_command_string();
        let curl_json: String = serde_json::to_string(&curl_cmd).unwrap_or_default();
        saved_commands.push(SavedCommand::new(&cmd, &curl_json, None));
        saved_commands
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Info {
    pub name: String,
    schema: String,
}
