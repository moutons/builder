// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

use config::SegmentCfg;
use error::{SegmentError, SegmentResult};
use serde_json;

use reqwest::header::{qitem, Accept, Authorization, Basic, ContentType, Headers, UserAgent};
use reqwest::mime;
use reqwest::Client;
use reqwest::{Proxy, Response};

const USER_AGENT: &'static str = "Habitat-Builder";

#[derive(Clone, Debug)]
pub struct SegmentClient {
    inner: Client,
    pub url: String,
    pub write_key: String,
}

impl SegmentClient {
    pub fn new(config: SegmentCfg) -> Self {
        let mut headers = Headers::new();
        headers.set(UserAgent::new(USER_AGENT));
        headers.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
        headers.set(ContentType(mime::APPLICATION_JSON));

        let mut client = Client::builder();
        client.default_headers(headers);

        if let Ok(url) = env::var("HTTP_PROXY") {
            debug!("Using HTTP_PROXY: {}", url);
            match Proxy::http(&url) {
                Ok(p) => {
                    client.proxy(p);
                }
                Err(e) => warn!("Invalid proxy url: {}, err: {:?}", url, e),
            }
        }

        if let Ok(url) = env::var("HTTPS_PROXY") {
            debug!("Using HTTPS_PROXY: {}", url);
            match Proxy::https(&url) {
                Ok(p) => {
                    client.proxy(p);
                }
                Err(e) => warn!("Invalid proxy url: {}, err: {:?}", url, e),
            }
        }

        SegmentClient {
            inner: client.build().unwrap(),
            url: config.url,
            write_key: config.write_key,
        }
    }

    pub fn identify(&self, user_id: &str) -> SegmentResult<Response> {
        let json = json!({ "userId": user_id });

        self.http_post(
            "identify",
            &self.write_key,
            serde_json::to_string(&json).unwrap(),
        )
    }

    pub fn track(&self, user_id: &str, event: &str) -> SegmentResult<Response> {
        let json = json!({
            "userId": user_id,
            "event": event
        });

        self.http_post(
            "track",
            &self.write_key,
            serde_json::to_string(&json).unwrap(),
        )
    }

    fn http_post(&self, path: &str, token: &str, body: String) -> SegmentResult<Response> {
        let url_path = format!("{}/v1/{}", &self.url, path);

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "".to_owned(),
            password: Some(token.to_owned()),
        }));

        self.inner
            .post(&url_path)
            .headers(headers)
            .body(body)
            .send()
            .map_err(SegmentError::HttpClient)
    }
}
