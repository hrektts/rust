// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
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
    libc.puts(rustrt.str_buf("hello, extern world 1"));
    libc.puts(rustrt.str_buf("hello, extern world 2"));
    libc.puts(rustrt.str_buf("hello, extern world 3"));
}
