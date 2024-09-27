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

extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::error::SgxStatus;
use sgx_types::types::*;
use sgx_urts::enclave::SgxEnclave;

static ENCLAVE_FILE: &str = "enclave.signed.so";
static SRS_PARAMS_FILE: &str = "/root/sgx/vendor/hyperplonk/hyperplonk/srs.params";

extern "C" {
    fn say_something(
        eid: EnclaveId,
        retval: *mut SgxStatus,
        some_string: *const u8,
        len: usize,
    ) -> SgxStatus;
}

use std::fs::File;
use std::io::Read;


fn main() {
    let enclave = match SgxEnclave::create(ENCLAVE_FILE, true) {
        Ok(enclave) => {
            println!("[+] Init Enclave Successful {}!", enclave.eid());
            enclave
        }
        Err(err) => {
            println!("[-] Init Enclave Failed {}!", err.as_str());
            return;
        }
    };

    let mut retval = SgxStatus::Success;

    let mut srs_file = File::open(SRS_PARAMS_FILE).unwrap();
    let mut buffer = Vec::new();
    srs_file.read_to_end(&mut buffer).unwrap();

    let result = unsafe {
        say_something(
            enclave.eid(),
            &mut retval,
            buffer.as_ptr() as *const u8,
            buffer.len(),
        )
    };
    match result {
        SgxStatus::Success => println!("[+] ECall Success..."),
        _ => println!("[-] ECall Enclave Failed {}!", result.as_str()),
    }
}
