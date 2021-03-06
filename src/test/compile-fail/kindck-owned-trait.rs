// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[feature(managed_boxes)];

trait foo { fn foo(&self); }

fn to_foo<T:Clone + foo>(t: T) -> @foo {
    @t as @foo
    //~^ ERROR value may contain borrowed pointers; add `'static` bound
    //~^^ ERROR cannot pack type
    //~^^^ ERROR value may contain borrowed pointers
}

fn to_foo2<T:Clone + foo + 'static>(t: T) -> @foo {
    @t as @foo
}

fn main() {}
