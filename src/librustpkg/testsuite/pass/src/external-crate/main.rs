// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
The test runner should check that, after `rustpkg install external crate`
  with RUST_PATH undefined in the environment
  and with `rustpkg install deeply/nested/path/foo` already
     executed:
   * ../bin/external_crate exists and is an executable

  tjc: Also want a test like this where foo is an external URL,
    which requires the `extern mod` changes
*/

extern mod foo;

fn main() {}
