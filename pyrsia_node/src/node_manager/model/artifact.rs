/*
   Copyright 2021 JFrog Ltd

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
extern crate pyrsia_client_lib;
extern crate serde_json;

use super::HashAlgorithm;
use pyrsia_client_lib::signed::Signed;
use serde_json::{Map, Value};

use signed_struct::signed_struct;

#[signed_struct]
#[derive(Debug)]
pub struct Artifact<'a> {
    hash: &'a [u8],
    algorithm: HashAlgorithm,
    name: Option<String>,
    creation_time: Option<String>,
    url: Option<String>,
    size: u32,
    mime_type: Option<String>,
    metadata: Map<String, Value>,
    source_url: Option<String>,
    art_type: Option<String>,
}