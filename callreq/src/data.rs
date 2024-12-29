// SONIC: Toolchain for formally-verifiable distributed contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2019-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use core::str::FromStr;
use std::convert::Infallible;

use amplify::confinement::{ConfinedVec, TinyBlob};
use chrono::{DateTime, Utc};
use hypersonic::{AuthToken, CallState, ContractId};
use strict_types::{StrictVal, TypeName};

/// Call request provides information for constructing [`hypersonic::CallParams`].
///
/// Request doesn't specify the used capabilities of the contract (blockchain, if any; type of
/// single-use seals) since each contract is strictly committed and can be used under one and just
/// one type of capabilities.
///
/// # URI form
///
/// Call request can be represented as a URI using `contract:` scheme in the following format:
///
/// ```text
/// contract:CONTRACT-ID/API/METHOD/STATE/AUTH/DATA+STON?expiry=DATETIME&lock=BASE64&endpoints=E1,
/// E2#CHECK
/// ```
///
/// NB: Parsing and producing URI form requires use of `uri` feature.
///
/// ## Path
///
/// Some path components of the URI may be skipped. In this case URI is parsed in the following way:
/// - 3-component path, starting with `/`, provides name of the used interface standard,
///   authentication token and state information;
/// - 3-component path, not starting with `/`, provides contract ID and auth token, and should use a
///   default method and name state from the contract default API;
/// - 4-component path - contract ID and state name are given in addition to the auth token, a
///   default method used from the contract default API;
/// - 5-component path - all parameters except API name are given.
///
/// ## Query
///
/// Supported URI query parameters are:
/// - `expiry`: ISO-8601 datetime string;
/// - `lock`: Base64-encoded lock script conditions;
/// - `endpoints`: comma-separated URLs with the endpoints for uploading a resulting
///   deeds/consignment stream.
///
/// ## Fragment
///
/// Optional fragment may be present and should represent a checksum value for the URI string
/// preceding the fragment.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CallRequest {
    pub contract_id: Option<ContractId>,
    pub api: Option<TypeName>,
    pub call: Option<CallState>,
    pub auth: AuthToken,
    pub data: StrictVal,
    pub lock: Option<TinyBlob>,
    pub expiry: Option<DateTime<Utc>>,
    pub endpoints: ConfinedVec<Endpoint, 0, 10>,
}

#[derive(Clone, Eq, PartialEq, Debug, Display)]
#[display(inner)]
#[non_exhaustive]
pub enum Endpoint {
    JsonRpc(String),
    RestHttp(String),
    WebSockets(String),
    Storm(String),
    UnspecifiedMeans(String),
}

impl FromStr for Endpoint {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(Endpoint::RestHttp(s))
        } else if s.starts_with("http+json-rpc://") || s.starts_with("https+json-rpc://") {
            Ok(Endpoint::RestHttp(s))
        } else if s.starts_with("ws://") || s.starts_with("wss://") {
            Ok(Endpoint::WebSockets(s))
        } else if s.starts_with("storm://") {
            Ok(Endpoint::Storm(s))
        } else {
            Ok(Endpoint::UnspecifiedMeans(s.to_string()))
        }
    }
}