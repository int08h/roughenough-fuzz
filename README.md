# roughenough-fuzz

[![Apache License 2](https://img.shields.io/badge/license-ASF2-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.txt)

This crate is for fuzzing [Roughenough](https://github.com/int08h/roughenough), a Rust 
[Roughtime](https://roughtime.googlesource.com/roughtime) secure time synchronization client and 
server implementation in Rust. 

## Building and Running

Fuzzing uses [American fuzzy lop](http://lcamtuf.coredump.cx/afl/) via [afl.rs](https://github.com/rust-fuzz/afl.rs).
Follow the [afl.rs setup instructions](https://rust-fuzz.github.io/book/afl/setup.html) to get `cargo afl` installed.

On Linux everything should "just work": 

```
$ cd roughenough-fuzz
$ cargo afl build
$ cargo afl fuzz -i in -o out -x dictionary.txt target/debug/roughenough-fuzz
```

On MacOS AFL will not compile on stable Rust 1.24.1. You will need to use 
`beta` or `nightly` to execute the fuzzing. `rustup` is your friend:

```
$ rustup run nightly cargo afl fuzz -i in -o out -x dictionary.txt target/debug/roughenough-fuzz
```

## Links
* [Roughenough Github repo](https://github.com/int08h/roughenough)
* [Roughtime project](https://roughtime.googlesource.com/roughtime)

## Contributors
* Stuart Stock (stuart {at} int08h.com)

## Copyright and License
roughenough-fuzz is copyright (c) 2018 int08h LLC. All rights reserved. 

int08h LLC licenses Roughenough (the "Software") to you under the Apache License, version 2.0 
(the "License"); you may not use this Software except in compliance with the License. You may obtain 
a copy of the License from the [LICENSE](../master/LICENSE) file included with the Software or at:

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License 
is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or 
implied. See the License for the specific language governing permissions and limitations under 
the License.
