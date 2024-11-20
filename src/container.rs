// SONIC: Toolchain for formally-verifiable distributed contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2024 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2019-2025 LNP/BP Standards Association, Switzerland.
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

use aluvm::Lib;
use amplify::confinement::{LargeVec, SmallOrdMap, SmallOrdSet, TinyOrdMap};
use commit_verify::ReservedBytes;
use sonicapi::Api;
use strict_encoding::TypeName;
use strict_types::TypeSystem;
use ultrasonic::{Codex, Operation, ProofOfPubl};

use crate::annotations::Annotations;
use crate::sigs::ContentSigs;
use crate::Contract;

pub type CodexContainer = Container<()>;
pub type ContractContainer<PoP> = Container<ContractExt<PoP>>;

pub struct Container<Ext> {
    pub codex: Codex,
    pub ext: Ext,
    pub apis: SmallOrdMap<Api, ContentSigs>,
    pub libs: SmallOrdSet<Lib>,
    pub types: TypeSystem,
    pub codex_sigs: ContentSigs,
    pub annotations: TinyOrdMap<Annotations, ContentSigs>,
    pub reserved: ReservedBytes<8>,
}

pub struct ContractExt<PoP: ProofOfPubl> {
    pub contract: Contract<PoP>,
    pub operations: LargeVec<Operation>,
    pub contract_sigs: ContentSigs,
}

impl Container<()> {
    pub fn issue<PoP: ProofOfPubl>(&self, api: Option<TypeName>) -> Contract<PoP> { todo!() }
}