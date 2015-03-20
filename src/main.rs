// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![feature(libc)]

extern crate libc;
extern crate winapi;

use libc::c_void;
use winapi::{ REFIID, HRESULT };
use dxgi_interfaces::*;
use dxgi_structures::*;
use dxgi_constants::*;

mod macros;
mod dxgi_structures;
mod dxgi_enumerations;
mod dxgi_constants;
mod dxgi_interfaces;

#[link(name="dxgi")]
extern "C" {
	#![allow(dead_code)]

	fn CreateDXGIFactory1(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
}

fn main() {
	use std::ptr;
	let mut factory: *mut c_void = ptr::null_mut();
	assert_eq!(0, unsafe { CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory) });
}