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

pub use github_api_client::types::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct JobCreateReq {
    pub project_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectCreateReq {
    pub origin: String,
    pub plan_path: String,
    pub installation_id: u32,
    pub repo_id: u32,
    pub auto_build: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectUpdateReq {
    pub plan_path: String,
    pub installation_id: u32,
    pub repo_id: u32,
    pub auto_build: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserUpdateReq {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupPromoteReq {
    pub idents: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupDemoteReq {
    pub idents: Vec<String>,
}