// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![cfg_attr(not(target_vendor = "teaclave"), no_std)]
#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]

#[cfg(not(target_vendor = "teaclave"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;

extern crate hyperplonk;

use sgx_types::error::SgxStatus;

mod hyperplonk_bench;
mod jf_plonk_bench;

#[no_mangle]
pub unsafe extern "C" fn say_something(_: *const u8, _: usize) -> SgxStatus {
    // hyperplonk_bench::bench()
    jf_plonk_bench::bench()
}

/*
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn say_something(some_string: *const u8, some_len: usize) -> SgxStatus {
    println!("hello from the other side!");

    SgxStatus::Success
}
    */
