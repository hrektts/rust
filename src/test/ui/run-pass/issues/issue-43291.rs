// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
pub fn main() {
    assert_eq!(!0usize as *const (), foo(0, 1));
    assert_eq!(!0usize as *const (), (0i8 - 1) as *const ());
}

pub fn foo(a: i8, b: i8) -> *const () {
    (a - b) as *const ()
}
