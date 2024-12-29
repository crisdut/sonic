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

use chrono::{DateTime, Utc};
use hypersonic::{AuthToken, CallState, MethodName, StateName};
use strict_types::{StrictVal, TypeName};

use crate::{CallRequest, Endpoint};

impl<T> CallRequest<T> {
    pub fn new(scope: T, auth: AuthToken, data: StrictVal) -> Self {
        Self {
            scope,
            api: None,
            call: None,
            auth,
            data,
            lock: None,
            expiry: None,
            endpoints: Default::default(),
            unknown_query: Default::default(),
        }
    }

    pub fn use_api(mut self, api: impl Into<TypeName>) -> Self {
        self.api = Some(api.into());
        self
    }

    pub fn use_method(mut self, method: MethodName) -> Self {
        if let Some(call) = &mut self.call {
            call.method = method;
        } else {
            self.call = Some(CallState::new(method));
        }
        self
    }

    pub fn use_state(mut self, state: StateName) -> Self {
        let mut call = self
            .call
            .expect("use_method must be called before use_state");
        call.destructible = Some(state);
        self.call = Some(call);
        self
    }

    pub fn use_expiry(mut self, expiry: DateTime<Utc>) -> Self {
        self.expiry = Some(expiry);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoints.push(endpoint);
        self
    }
}