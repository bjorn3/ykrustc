// force-host
// no-prefer-dynamic
// edition:2018

#![feature(proc_macro_def_site)]
#![crate_type = "proc-macro"]

extern crate proc_macro;
extern crate make_macro;
use proc_macro::{TokenStream, Span};

make_macro::make_it!(print_def_site);
