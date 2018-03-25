# roughenough-fuzz

[![Apache License 2](https://img.shields.io/badge/license-ASF2-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.txt)

This crate is for fuzzing [Roughenough](https://github.com/int08h/roughenough), a Rust
implementation of the [Roughtime](https://roughtime.googlesource.com/roughtime) secure time 
synchronization protocol. 

## Building and Installation

Fuzzing uses [American fuzzy lop](http://lcamtuf.coredump.cx/afl/) via [afl.rs](https://github.com/rust-fuzz/afl.rs),
[libFuzzer](http://llvm.org/docs/LibFuzzer.html) via [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz),
and [honggfuzz](http://honggfuzz.com/) via [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).

**Rust Nightly** is needed to compile this crate. The nightly-only `-Z` compiler flag is required
by `cargo-fuzz`. MacOS users also need nightly to compile `afl.rs`.

To install:

* `cargo install afl cargo-fuzz honggfuzz`

FYI, Ubuntu 17.10 needed the `binutils-dev` and `libunwind-dev` packages to compile cargo-fuzz. YMMV.

## Running

```
$ cd roughenough-fuzz

# Run AFL
$ cargo afl build --release
$ cargo afl fuzz -i in -o out -x dictionary.txt target/release/roughenough-fuzz

# Run libFuzzer
$ cargo fuzz run -O fuzz_target_1 -- -dict=dictionary.txt -max_len=2048

# Run honggfuzz
$ HFUZZ_RUN_ARGS="--dict dictionary.txt --max_file_size 2048 --input in/" cargo hfuzz run hfuzz_target
```

## Links

* [Roughenough Github repo](https://github.com/int08h/roughenough)
* [Roughtime project](https://roughtime.googlesource.com/roughtime)
* [Rust Fuzzing Authority](https://github.com/rust-fuzz)
* [libFuzzer Tutorial](https://github.com/google/fuzzer-test-suite/blob/master/tutorial/libFuzzerTutorial.md)

## Contributors
* Stuart Stock (stuart {at} int08h.com)

## Copyright and License
roughenough-fuzz is copyright (c) 2018 int08h LLC. All rights reserved. 

int08h LLC licenses roughenough-fuzz (the "Software") to you under the Apache License, version 2.0 
(the "License"); you may not use this Software except in compliance with the License. You may obtain 
a copy of the License from the [LICENSE](../master/LICENSE) file included with the Software or at:

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License 
is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or 
implied. See the License for the specific language governing permissions and limitations under 
the License.
