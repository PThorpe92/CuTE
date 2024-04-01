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
                if let Some(method) = request.get("method").unwrap().as_str() {
                    let method = method.to_uppercase();
                    match method.as_str() {
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
                if let Some(url) = request.get("url") {
                    if let Some(url) = url.as_object() {
                        if let Some(url) = url.get("raw") {
                            let url = url.as_str().unwrap();
                            curl_cmd.set_url(url);
                        }
                    }
                }
                if let Some(cookie) = request.get("cookie") {
                    if let Some(cookie) = cookie.as_array() {
                        cookie.iter().for_each(|ck| {
                            if let Some(ck) = ck.as_object() {
                                let key = ck.get("key").unwrap().as_str().unwrap();
                                let value = ck.get("value").unwrap().as_str().unwrap();
                                curl_cmd.add_cookie(&format!("{}: {}", key, value));
                            }
                        });
                    }
                }
                if let Some(headers) = request.get("header") {
                    let headers = headers.as_array().unwrap();
                    headers.iter().for_each(|hdr| {
                        if let Some(hdr) = hdr.as_object() {
                            if let Some(key) = hdr.get("key") {
                                let key = key.as_str().unwrap();
                                if let Some(value) = hdr.get("value") {
                                    let value = value.as_str().unwrap();
                                    curl_cmd.add_headers(&format!("{}: {}", key, value));
                                }
                            }
                        }
                    });
                }
            }
            if let Some(response) = item.get("response") {
                if let Some(response) = response.as_array() {
                    response.iter().for_each(|resp| {
                        if let Some(resp) = resp.as_object() {
                            if let Some(body) = resp.get("body") {
                                if let Some(body) = body.get("raw") {
                                    let body = body.as_str().unwrap();
                                    curl_cmd.set_request_body(body);
                                }
                            }
                            if resp.get("url").is_some() && !curl_cmd.get_url().is_empty() {
                                let url = resp.get("url").unwrap().as_object().unwrap();
                                let url = url.get("raw").unwrap().as_str().unwrap();
                                curl_cmd.set_url(url);
                            }
                            if let Some(cookie) = resp.get("cookie") {
                                if let Some(cookie) = cookie.as_array() {
                                    cookie.iter().for_each(|ck| {
                                        if let Some(ck) = ck.as_object() {
                                            let key = ck.get("key").unwrap().as_str().unwrap();
                                            let value = ck.get("value").unwrap().as_str().unwrap();
                                            curl_cmd.add_cookie(&format!("{}: {}", key, value));
                                        }
                                    });
                                }
                            }
                        }
                    });
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
    _postman_id: String,
    pub name: String,
    schema: String,
}
