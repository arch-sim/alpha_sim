//     Copyright 2019 Haoran Wang
//
//     Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

pub type InstType = u32;
pub type AddrType = u64;
pub type DataType = u64;
pub type data_t = u64;

pub const PROJ: &'static str = env!("CARGO_PKG_NAME");
pub const NAME: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const VER: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const LICENSE: &'static str = "Apache License 2.0";
