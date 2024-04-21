use super::db::SavedCommand;
use crate::request::curl::{Curl, Method};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};

#[derive(Serialize, Debug, Deserialize)]
pub struct PostmanCollection {
    pub info: Info,
    pub item: Vec<HashMap<String, Value>>,
}

impl From<PostmanCollection> for Vec<SavedCommand> {
    fn from(collection: PostmanCollection) -> Vec<SavedCommand> {
        let mut saved_commands = Vec::new();
        collection.item.iter().for_each(|item| {
            let mut curl_cmd = Curl::new_serializing();
            let mut cmd_name: Option<String> = None;
            let mut description: Option<String> = None;
            if let Some(name) = item.get("name") {
                if let Some(name) = name.as_str() {
                    cmd_name = Some(name.to_string());
                }
            }
            if let Some(request) = item.get("request") {
                if let Some(request) = request.as_str() {
                    // this means its a get request
                    curl_cmd.set_url(request);
                    curl_cmd.set_get_method();
                } else if let Some(request) = request.as_object() {
                    if let Some(desc) = request.get("description") {
                        if let Some(desc) = desc.as_str() {
                            description = Some(desc.to_string());
                        }
                    }
                    if let Some(url) = request.get("url") {
                        if let Some(str_url) = url.as_str() {
                            curl_cmd.set_url(str_url);
                        } else if let Some(url) = url.as_object() {
                            if let Some(raw) = url.get("raw") {
                                if let Some(raw_str) = raw.as_str() {
                                    curl_cmd.set_url(raw_str);
                                }
                            }
                        }
                    }
                    if let Some(method) = request.get("method") {
                        if let Some(method) = method.as_str() {
                            curl_cmd.set_method(Method::from_str(method).unwrap_or_default());
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
                    if let Some(cookie) = request.get("cookie") {
                        if let Some(cookie) = cookie.as_array() {
                            cookie.iter().for_each(|ck| {
                                if let Some(ck) = ck.as_object() {
                                    if let Some(key) = ck.get("key") {
                                        if let Some(key) = key.as_str() {
                                            if let Some(value) = ck.get("value") {
                                                if let Some(value) = value.as_str() {
                                                    curl_cmd
                                                        .add_cookie(&format!("{}: {}", key, value));
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
            if cmd_name.is_none() {
                cmd_name = Some(String::from(curl_cmd.get_url()));
            }
            let cmd = curl_cmd.get_command_string();
            let curl_json: String = serde_json::to_string(&curl_cmd).unwrap_or_default();
            saved_commands.push(SavedCommand::new(
                &cmd,
                cmd_name,
                description,
                &curl_json,
                None,
            ));
        });

        saved_commands
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Info {
    pub name: String,
    schema: String,
    pub description: String,
}
