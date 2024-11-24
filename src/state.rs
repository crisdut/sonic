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

use alloc::collections::BTreeMap;

use indexmap::IndexMap;
use sonicapi::{Api, StateAtom, StateName};
use strict_encoding::TypeName;
use strict_types::{StrictVal, TypeSystem};
use ultrasonic::{fe128, CellAddr, Memory, Operation, StateCell, StateData, StateValue};

#[derive(Clone, Debug, Default)]
pub struct ContractState {
    pub raw: RawState,
    pub main: AdaptedState,
    pub apis: BTreeMap<TypeName, AdaptedState>,
}

#[derive(Clone, Debug, Default)]
pub struct RawState {
    pub seals: IndexMap<fe128, CellAddr>,
    pub immutable: BTreeMap<CellAddr, StateData>,
    pub owned: BTreeMap<CellAddr, StateCell>,
}

impl Memory for RawState {
    fn read_once(&self, addr: CellAddr) -> Option<StateCell> { self.owned.get(&addr).copied() }
    fn immutable(&self, addr: CellAddr) -> Option<StateValue> { self.immutable.get(&addr).map(|data| data.value) }
}

impl RawState {
    pub fn seal_addr(&self, seal: fe128) -> CellAddr { *self.seals.get(&seal).expect("undefined seal") }

    pub fn apply(&mut self, op: Operation) {
        let opid = op.opid();
        for input in op.destroying {
            let res = self.owned.remove(&input.addr).expect("unknown input");
            self.seals.shift_remove(&res.seal);
        }
        for (no, cell) in op.destructible.into_iter().enumerate() {
            let addr = CellAddr::new(opid, no as u16);
            self.seals.insert(cell.seal, addr);
            self.owned.insert(addr, cell);
        }
        self.immutable.extend(
            op.immutable
                .into_iter()
                .enumerate()
                .map(|(no, data)| (CellAddr::new(opid, no as u16), data)),
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct AdaptedState {
    pub immutable: BTreeMap<StateName, BTreeMap<CellAddr, StateAtom>>,
    pub owned: BTreeMap<StateName, BTreeMap<CellAddr, StrictVal>>,
}

impl AdaptedState {
    pub fn immutable(&self, name: &StateName) -> Option<&BTreeMap<CellAddr, StateAtom>> { self.immutable.get(name) }

    pub fn owned(&self, name: &StateName) -> Option<&BTreeMap<CellAddr, StrictVal>> { self.owned.get(name) }

    pub fn apply(&mut self, op: &Operation, api: &Api, sys: &TypeSystem) {
        let opid = op.opid();
        for (no, state) in op.immutable.iter().enumerate() {
            if let Some((name, atom)) = api.convert_immutable(state, sys) {
                self.immutable
                    .entry(name)
                    .or_default()
                    .insert(CellAddr::new(opid, no as u16), atom);
            }
        }
        for input in &op.destroying {
            for map in self.owned.values_mut() {
                map.remove(&input.addr);
            }
        }
        for (no, state) in op.destructible.iter().enumerate() {
            if let Some((name, atom)) = api.convert_destructible(state.data, sys) {
                self.owned
                    .entry(name)
                    .or_default()
                    .insert(CellAddr::new(opid, no as u16), atom);
            }
        }
    }
}